//! Download and compile Tree-sitter grammars.

use std::fs;
use std::process::Command;

use crate::runtime::Runtime;
use crate::grammar_registry;

/// Build a Tree-sitter grammar and install it to the runtime directory
pub fn build_and_install_grammar(language_id: &str) -> Result<(), String> {
    let grammar_info = grammar_registry::for_language(language_id)
        .ok_or_else(|| format!("No grammar info available for {}", language_id))?;
    
    let runtime = Runtime::new();
    
    // Create temporary directory
    let temp_dir = tempfile::tempdir()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Download source as zip instead of using git clone
    println!("Downloading {} grammar source...", language_id);
    let repo_dir = temp_dir.path().join("repo");
    
    // Create repo directory
    fs::create_dir_all(&repo_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", repo_dir.display(), e))?;
    
    // Use git clone with timeout and no credential helper
    println!("Cloning {}...", grammar_info.repo_url);
    
    // Set up git command with timeout and no credential helper
    let mut cmd = Command::new("timeout");
    cmd.args(["30", "git", "clone", "--depth", "1"]);
    
    // Disable credential helper to avoid prompts
    cmd.args(["--config", "credential.helper="]);
    
    cmd.args([&grammar_info.repo_url, repo_dir.to_str().unwrap()]);
    
    match cmd.status() {
        Ok(status) if status.success() => {
            println!("Successfully cloned repository");
        }
        Ok(_) => {
            // Try without timeout command (for systems without timeout)
            let mut cmd2 = Command::new("git");
            cmd2.args(["clone", "--depth", "1", "--config", "credential.helper=", &grammar_info.repo_url, repo_dir.to_str().unwrap()]);
            
            match cmd2.status() {
                Ok(status2) if status2.success() => {
                    println!("Successfully cloned repository");
                }
                Ok(_) => {
                    return Err("Failed to clone repository (git clone failed)".to_string());
                }
                Err(e) => {
                    return Err(format!("Failed to run git clone: {}", e));
                }
            }
        }
        Err(_e) => {
            // timeout command not available, try git directly
            let mut cmd2 = Command::new("git");
            cmd2.args(["clone", "--depth", "1", "--config", "credential.helper=", &grammar_info.repo_url, repo_dir.to_str().unwrap()]);
            
            match cmd2.status() {
                Ok(status2) if status2.success() => {
                    println!("Successfully cloned repository");
                }
                Ok(_) => {
                    return Err("Failed to clone repository (git clone failed)".to_string());
                }
                Err(e2) => {
                    return Err(format!("Failed to run git clone: {}", e2));
                }
            }
        }
    }
    
    // No zip extraction needed - we cloned directly into repo_dir
    
    // We cloned directly into repo_dir, so source_dir is repo_dir
    // Navigate to subdirectory if needed
    let source_dir = if let Some(subdir) = &grammar_info.subdirectory {
        repo_dir.join(subdir)
    } else {
        repo_dir.clone()
    };
    
    // Verify source directory exists
    if !source_dir.exists() {
        return Err(format!("Source directory does not exist: {:?}", source_dir));
    }
    
    // Check if tree-sitter CLI is available
    let has_tree_sitter_cli = Command::new("tree-sitter")
        .arg("--version")
        .output()
        .is_ok();
    
    let lib_path;
    
    if has_tree_sitter_cli {
        // Use tree-sitter CLI
        println!("Using tree-sitter CLI to build {}...", language_id);
        
        // Check if package.json exists and install dependencies if needed
        if source_dir.join("package.json").exists() {
            println!("Installing npm dependencies...");
            let install_output = Command::new("npm")
                .current_dir(&source_dir)
                .arg("install")
                .output()
                .map_err(|e| format!("Failed to run npm install: {}", e))?;
            
            if !install_output.status.success() {
                let stderr = String::from_utf8_lossy(&install_output.stderr);
                let stdout = String::from_utf8_lossy(&install_output.stdout);
                eprintln!("npm install output:\nstdout: {}\nstderr: {}", stdout, stderr);
                // Continue anyway, maybe dependencies are already installed
            } else {
                println!("npm dependencies installed successfully");
            }
        }
        
        // Run tree-sitter generate and capture output
        let generate_output = Command::new("tree-sitter")
            .current_dir(&source_dir)
            .arg("generate")
            .output()
            .map_err(|e| format!("Failed to run tree-sitter generate: {}", e))?;
        
        if !generate_output.status.success() {
            let stderr = String::from_utf8_lossy(&generate_output.stderr);
            let stdout = String::from_utf8_lossy(&generate_output.stdout);
            eprintln!("tree-sitter generate failed, trying with npx...");
            
            // Try with npx tree-sitter generate
            let npx_output = Command::new("npx")
                .current_dir(&source_dir)
                .args(["tree-sitter", "generate"])
                .output()
                .map_err(|e| format!("Failed to run npx tree-sitter generate: {}", e))?;
            
            if !npx_output.status.success() {
                let npx_stderr = String::from_utf8_lossy(&npx_output.stderr);
                let npx_stdout = String::from_utf8_lossy(&npx_output.stdout);
                return Err(format!("tree-sitter generate failed with both tree-sitter CLI and npx:\nFirst error:\nstdout: {}\nstderr: {}\n\nNpx error:\nstdout: {}\nstderr: {}", 
                    stdout, stderr, npx_stdout, npx_stderr));
            }
            println!("tree-sitter generate succeeded with npx");
        }
        
        // Run tree-sitter build and capture output
        let build_output = Command::new("tree-sitter")
            .current_dir(&source_dir)
            .arg("build")
            .output()
            .map_err(|e| format!("Failed to run tree-sitter build: {}", e))?;
        
        if !build_output.status.success() {
            let stderr = String::from_utf8_lossy(&build_output.stderr);
            let stdout = String::from_utf8_lossy(&build_output.stdout);
            return Err(format!("tree-sitter build failed:\nstdout: {}\nstderr: {}", stdout, stderr));
        }
        
        // Print build output for debugging
        let build_stdout = String::from_utf8_lossy(&build_output.stdout);
        let build_stderr = String::from_utf8_lossy(&build_output.stderr);
        if !build_stdout.trim().is_empty() {
            println!("tree-sitter build stdout: {}", build_stdout);
        }
        if !build_stderr.trim().is_empty() {
            println!("tree-sitter build stderr: {}", build_stderr);
        }
        
        // Find the built library - tree-sitter CLI may place it in several locations
        // tree-sitter build typically creates a file named parser.so (or parser.dylib/parser.dll)
        // in the source directory, not libtree-sitter-{language}.so
        let lib_name = get_library_name(language_id);
        
        // First, look for the standard tree-sitter CLI output: parser.{so,dylib,dll}
        let parser_lib_name = if cfg!(windows) {
            "parser.dll"
        } else if cfg!(target_os = "macos") {
            "parser.dylib"
        } else {
            "parser.so"
        };
        
        // Possible locations where tree-sitter CLI might place the library
        // tree-sitter build typically creates a file named parser.so (or parser.dylib/parser.dll)
        // in various locations
        let possible_paths = vec![
            source_dir.join(parser_lib_name),                     // parser.so in source directory
            source_dir.join(&lib_name),                           // libtree-sitter-{language}.so in source directory
            source_dir.join("target").join("release").join(parser_lib_name), // target/release/parser.so
            source_dir.join("target").join("release").join(&lib_name), // target/release/libtree-sitter-{language}.so
            source_dir.join("target").join(parser_lib_name),      // target/parser.so
            source_dir.join("target").join(&lib_name),            // target/libtree-sitter-{language}.so
            source_dir.join("out").join(parser_lib_name),         // out/parser.so (some grammars)
            source_dir.join("out").join(&lib_name),               // out/libtree-sitter-{language}.so (some grammars)
            source_dir.join("build").join(parser_lib_name),       // build/parser.so (some grammars)
            source_dir.join("build").join(&lib_name),             // build/libtree-sitter-{language}.so (some grammars)
        ];
        
        // Also check for debug builds
        let debug_paths = vec![
            source_dir.join("target").join("debug").join(parser_lib_name),
            source_dir.join("target").join("debug").join(&lib_name),
        ];
        let all_paths: Vec<_> = possible_paths.into_iter().chain(debug_paths.into_iter()).collect();
        
        let mut found = None;
        println!("Checking possible library paths:");
        for path in &all_paths {
            println!("  Checking: {}", path.display());
            if path.exists() {
                println!("Found library at: {}", path.display());
                found = Some(path.clone());
                break;
            }
        }
        
        if let Some(found_path) = found {
            lib_path = found_path;
        } else {
            // If not found, try to list files to debug
            println!("Searching for library {} or {} in {}...", parser_lib_name, lib_name, source_dir.display());
            if let Ok(entries) = std::fs::read_dir(&source_dir) {
                for entry in entries.flatten() {
                    println!("  Found: {}", entry.path().display());
                }
            }
            
            // Also check target directory
            let target_dir = source_dir.join("target");
            if target_dir.exists() {
                println!("Checking target directory...");
                if let Ok(entries) = std::fs::read_dir(&target_dir) {
                    for entry in entries.flatten() {
                        println!("  Found in target: {}", entry.path().display());
                    }
                }
            }
            
            return Err(format!("Could not find built library {} or {} after tree-sitter build. Searched in: {:?}", 
                parser_lib_name, lib_name, all_paths));
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
                println!("Warning: Source file {} does not exist, skipping", source_file);
                continue; // Skip missing files (some grammars don't have scanner.c)
            }
            
            let object_file = temp_dir.path().join(format!("{}.o", source_file.replace('/', "_")));
            
            println!("Compiling {}...", source_file);
            let output = Command::new("cc")
                .args(["-c", "-fPIC", "-I./src", "-o"])
                .arg(&object_file)
                .arg(&source_path)
                .current_dir(&source_dir)
                .output()
                .map_err(|e| format!("Failed to compile {}: {}", source_file, e))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to compile {}: {}", source_file, stderr));
            }
            
            object_files.push(object_file);
        }
        
        if object_files.is_empty() {
            return Err("No source files compiled".to_string());
        }
        
        // Link shared library
        let lib_name = get_library_name(language_id);
        lib_path = temp_dir.path().join(&lib_name);
        
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
    }
    
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
