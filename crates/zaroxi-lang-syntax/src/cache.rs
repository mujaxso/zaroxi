//! Syntax‑highlight cache for the editor.
//!
//! Stores parsed trees and highlight spans keyed by canonical file path
//! and document version.  This avoids rebuilding the entire syntax tree
//! for every scroll event or tab activation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use once_cell::sync::Lazy;

use crate::language::LanguageId;
use crate::parser::{ParserPool, SyntaxTree};
use crate::highlight::{HighlightEngine, HighlightSpan};

/// Cached syntax data for one document version.
struct CachedSyntax {
    tree: SyntaxTree,
    spans: Vec<HighlightSpan>,
    version: u64,
}

/// Per‑document syntax cache, keyed by canonical path.
static GLOBAL_CACHE: Lazy<Mutex<HashMap<PathBuf, CachedSyntax>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Return the cached highlight spans for the given path and version,
/// or build them from scratch if missing or stale.
pub fn get_or_compute(
    path: &PathBuf,
    version: u64,
    full_text: &str,
    lang: LanguageId,
    pool: Arc<ParserPool>,
    engine: &HighlightEngine,
) -> Result<Vec<HighlightSpan>, String> {
    let mut cache = GLOBAL_CACHE.lock();

    if let Some(entry) = cache.get(path) {
        if entry.version == version {
            return Ok(entry.spans.clone());
        }
    }

    let tree =
        SyntaxTree::new(pool, full_text, lang).map_err(|e| format!("Syntax error: {}", e))?;
    let spans = engine
        .highlight(lang, full_text, tree.tree())
        .map_err(|e| format!("Highlight error: {}", e))?;

    cache.insert(
        path.clone(),
        CachedSyntax {
            tree,
            spans: spans.clone(),
            version,
        },
    );

    Ok(spans)
}

/// Invalidate the cache for a given path (called after an edit).
pub fn invalidate(path: &PathBuf) {
    let mut cache = GLOBAL_CACHE.lock();
    cache.remove(path);
}
