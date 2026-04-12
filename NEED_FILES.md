To finish the Tree‑sitter runtime integration and **bundle grammars statically**, we need to edit the following files in `crates/syntax‑core/`. Please add them to the chat:

1. `crates/syntax‑core/Cargo.toml`
2. `crates/syntax‑core/src/language.rs`
3. `crates/syntax‑core/src/manager.rs`
4. `crates/syntax‑core/src/highlight.rs`
5. `crates/syntax‑core/src/runtime.rs`
6. `crates/syntax‑core/src/metadata.rs`
7. `crates/syntax‑core/src/lib.rs`

Once you’ve added these files, we’ll implement:
- Static linking of `tree‑sitter‑rust` and `tree‑sitter‑toml` crates (no external `.so` files).
- Embedding of query strings via `include_str!` from the `runtime/treesitter/` directory.
- Proper highlight‑span generation and a clean API for the UI.
- Removal of the external grammar‑fetching script (since grammars will be built at compile time).
