//! Download and compile Tree-sitter grammars.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::runtime::Runtime;
use super::grammar_registry::GrammarInfo;

/// Build a Tree-sitter grammar and install it to the runtime directory
pub fn build_and_install_grammar(language_id: &str) -> Result<(), String> {
    let grammar_info = GrammarInfo::for_language(language_id)
        .ok_or_else(|| format!("No grammar info available for {}", language_id))?;
    
    let runtime = Runtime::new();
    
    // Create temporary directory
    let temp_dir = tempfile::tempdir()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Download source as zip instead of using git clone
    println!("Downloading {} grammar source...", language_id);
    let repo_dir = temp_dir.path().join("repo");
    
    // Create repo directory
    fs::create_dir_all(&repo_dir)?;
    
    // Download zip file from GitHub
    // Extract repo owner and name from the URL
    let repo_url = grammar_info.repo_url.trim_end_matches(".git");
    let parts: Vec<&str> = repo_url.split('/').collect();
    if parts.len() < 2 {
        return Err(format!("Invalid repo URL: {}", repo_url));
    }
    let repo_owner = parts[parts.len() - 2];
    let repo_name = parts[parts.len() - 1];
    
    // Use GitHub's archive URL which doesn't require authentication
    let zip_url = format!("https://github.com/{}/{}/archive/refs/heads/main.zip", repo_owner, repo_name);
    let zip_path = temp_dir.path().join("source.zip");
    
    // Try main branch first, then master as fallback
    let download_result = download_file(&zip_url, &zip_path);
    let zip_url = if download_result.is_err() {
        // Try master branch
        format!("https://github.com/{}/{}/archive/refs/heads/master.zip", repo_owner, repo_name)
    } else {
        zip_url
    };
    
    // Download the file
    download_file(&zip_url, &zip_path).map_err(|e| {
        format!("Failed to download source from {}: {}. Please install curl/wget.", zip_url, e)
    })?;
    
    // Extract zip
    let extract_ok = if cfg!(windows) {
        Command::new("powershell")
            .args(["-Command", &format!("Expand-Archive -Path {} -DestinationPath {}", zip_path.display(), repo_dir.display())])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    } else {
        Command::new("unzip")
            .args(["-q", zip_path.to_str().unwrap(), "-d", repo_dir.to_str().unwrap()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };
    
    if !extract_ok {
        return Err("Failed to extract source zip".to_string());
    }
    
    // Find the extracted directory (usually ends with -main or -master)
    let mut source_dir = None;
    for entry in fs::read_dir(&repo_dir).map_err(|e| format!("Failed to read repo dir: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                // Look for directories that contain the language name or are likely the source
                if dir_name.contains(language_id) || dir_name.contains("-main") || dir_name.contains("-master") {
                    source_dir = Some(path);
                    break;
                }
            }
        }
    }
    
    // If we didn't find a specific directory, use the first directory in repo_dir
    let source_dir = match source_dir {
        Some(dir) => dir,
        None => {
            // List all entries and find the first directory
            let mut entries: Vec<_> = fs::read_dir(&repo_dir)
                .map_err(|e| format!("Failed to read repo dir: {}", e))?
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .collect();
            
            if entries.is_empty() {
                return Err(format!("No directories found in {:?}", repo_dir));
            }
            
            // Sort to get a consistent result
            entries.sort_by_key(|entry| entry.path());
            entries[0].path()
        }
    };
    
    // Navigate to subdirectory if needed
    let source_dir = if let Some(subdir) = &grammar_info.subdirectory {
        source_dir.join(subdir)
    } else {
        source_dir
    };
    
    // Check if tree-sitter CLI is available
    let has_tree_sitter_cli = Command::new("tree-sitter")
        .arg("--version")
        .output()
        .is_ok();
    
    let lib_path = if has_tree_sitter_cli {
        // Use tree-sitter CLI
        println!("Using tree-sitter CLI to build {}...", language_id);
        
        let status = Command::new("tree-sitter")
            .current_dir(&source_dir)
            .arg("generate")
            .status()
            .map_err(|e| format!("Failed to run tree-sitter generate: {}", e))?;
        
        if !status.success() {
            return Err("tree-sitter generate failed".to_string());
        }
        
        let status = Command::new("tree-sitter")
            .current_dir(&source_dir)
            .arg("build")
            .status()
            .map_err(|e| format!("Failed to run tree-sitter build: {}", e))?;
        
        if !status.success() {
            return Err("tree-sitter build failed".to_string());
        }
        
        // Find the built library
        let target_dir = source_dir.join("target").join("release");
        let lib_name = get_library_name(language_id);
        
        if target_dir.join(&lib_name).exists() {
            target_dir.join(lib_name)
        } else if source_dir.join(&lib_name).exists() {
            source_dir.join(lib_name)
        } else {
            return Err(format!("Could not find built library {}", lib_name));
        }
    } else {
        // Manual compilation with cc
        println!("Using cc to build {}...", language_id);
        
        // Generate parser.c if needed
        if !source_dir.join("src/parser.c").exists() {
            if source_dir.join("grammar.js").exists() {
                // Try to use node with tree-sitter CLI package
                let status = Command::new("npx")
                    .current_dir(&source_dir)
                    .args(["tree-sitter", "generate"])
                    .status();
                
                if status.is_err() || !status.unwrap().success() {
                    return Err("Failed to generate parser.c. Install tree-sitter CLI or node.js".to_string());
                }
            } else {
                return Err("No grammar.js found and parser.c doesn't exist".to_string());
            }
        }
        
        // Compile all source files
        let mut object_files = Vec::new();
        for source_file in &grammar_info.source_files {
            let source_path = source_dir.join(source_file);
            if !source_path.exists() {
                continue; // Skip missing files (some grammars don't have scanner.c)
            }
            
            let object_file = temp_dir.path().join(format!("{}.o", source_file.replace('/', "_")));
            
            let status = Command::new("cc")
                .args(["-c", "-fPIC", "-I./src", "-o"])
                .arg(&object_file)
                .arg(&source_path)
                .current_dir(&source_dir)
                .status()
                .map_err(|e| format!("Failed to compile {}: {}", source_file, e))?;
            
            if !status.success() {
                return Err(format!("Failed to compile {}", source_file));
            }
            
            object_files.push(object_file);
        }
        
        if object_files.is_empty() {
            return Err("No source files compiled".to_string());
        }
        
        // Link shared library
        let lib_name = get_library_name(language_id);
        let lib_path = temp_dir.path().join(&lib_name);
        
        let mut cmd = Command::new("cc");
        cmd.args(["-shared", "-fPIC", "-o"])
            .arg(&lib_path);
        
        for obj in &object_files {
            cmd.arg(obj);
        }
        
        cmd.arg("-lstdc++");
        
        let status = cmd.status()
            .map_err(|e| format!("Failed to link library: {}", e))?;
        
        if !status.success() {
            return Err("Failed to link shared library".to_string());
        }
        
        lib_path
    };
    
    // Install library to runtime directory
    let target_dir = runtime.grammar_dir();
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;
    
    let target_lib_path = target_dir.join(get_library_name(language_id));
    
    fs::copy(&lib_path, &target_lib_path)
        .map_err(|e| format!("Failed to copy library: {}", e))?;
    
    println!("Installed library to: {}", target_lib_path.display());
    
    // Install query files
    let query_source_dir = source_dir.join("queries");
    if query_source_dir.exists() {
        let query_target_dir = runtime.language_dir(language_id).join("queries");
        fs::create_dir_all(&query_target_dir)
            .map_err(|e| format!("Failed to create query directory: {}", e))?;
        
        for query_file in &grammar_info.query_files {
            let source_path = query_source_dir.join(query_file);
            if source_path.exists() {
                let target_path = query_target_dir.join(query_file);
                fs::copy(&source_path, &target_path)
                    .map_err(|e| format!("Failed to copy query file {}: {}", query_file, e))?;
                println!("Installed query file: {}", query_file);
            }
        }
    } else {
        println!("Warning: No query directory found for {}", language_id);
    }
    
    println!("Successfully installed {} grammar!", language_id);
    Ok(())
}

/// Download a file from a URL to a local path
fn download_file(url: &str, path: &std::path::Path) -> Result<(), String> {
    use std::io::Write;
    
    // Try using ureq for HTTP requests (no authentication required)
    let response = ureq::get(url)
        .timeout_connect(10_000)  // 10 seconds
        .timeout_read(30_000)     // 30 seconds
        .timeout_write(10_000)    // 10 seconds
        .call();
    
    match response {
        Ok(resp) => {
            let mut file = std::fs::File::create(path)
                .map_err(|e| format!("Failed to create file {}: {}", path.display(), e))?;
            
            let mut reader = resp.into_reader();
            std::io::copy(&mut reader, &mut file)
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            
            Ok(())
        }
        Err(ureq::Error::Status(code, resp)) => {
            Err(format!("HTTP error {}: {}", code, resp.status_text()))
        }
        Err(ureq::Error::Status(code, resp)) => {
            Err(format!("HTTP error {}: {}", code, resp.status_text()))
        }
        Err(e) => {
            // Fall back to curl/wget if ureq fails
            eprintln!("ureq download failed: {}. Trying fallback...", e);
            fallback_download(url, path)
        }
    }
}

/// Fallback download using system commands
fn fallback_download(url: &str, path: &std::path::Path) -> Result<(), String> {
    let download_ok = if cfg!(windows) {
        Command::new("powershell")
            .args(["-Command", &format!("Invoke-WebRequest -Uri {} -OutFile {}", url, path.display())])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    } else {
        // Try curl first
        let curl_status = Command::new("curl")
            .args(["-L", "-o", path.to_str().unwrap(), url])
            .status();
        
        if curl_status.is_err() || !curl_status.unwrap().success() {
            // Try wget as fallback
            Command::new("wget")
                .args(["-O", path.to_str().unwrap(), url])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            true
        }
    };
    
    if download_ok {
        Ok(())
    } else {
        Err("Failed to download using curl/wget/powershell".to_string())
    }
}

/// Get the platform-specific library name for a language
fn get_library_name(language_id: &str) -> String {
    let prefix = if cfg!(windows) { "" } else { "lib" };
    let extension = if cfg!(windows) {
        ".dll"
    } else if cfg!(target_os = "macos") {
        ".dylib"
    } else {
        ".so"
    };
    
    format!("{}tree-sitter-{}{}", prefix, language_id, extension)
}

/// Check if a grammar is installed
pub fn is_grammar_installed(language_id: &str) -> bool {
    let runtime = Runtime::new();
    let lib_path = runtime.grammar_library_path(language_id);
    lib_path.exists()
}

/// Install missing grammars for a list of languages
pub fn install_missing_grammars(language_ids: &[&str]) -> Vec<String> {
    let mut installed = Vec::new();
    
    for &language_id in language_ids {
        if !is_grammar_installed(language_id) {
            println!("Grammar for {} is not installed. Installing...", language_id);
            match build_and_install_grammar(language_id) {
                Ok(()) => {
                    installed.push(language_id.to_string());
                    println!("Successfully installed {} grammar", language_id);
                }
                Err(e) => {
                    eprintln!("Failed to install {} grammar: {}", language_id, e);
                }
            }
        } else {
            println!("Grammar for {} is already installed", language_id);
        }
    }
    
    installed
}
