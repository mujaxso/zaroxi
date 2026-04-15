; Markdown injections for tree-sitter-markdown-inline
; Based on available node types from debug output

; HTML injection
((html_tag) @injection.content
  (#set! injection.language "html"))

; LaTeX injection
((latex_block) @injection.content
  (#set! injection.language "latex"))

; Note: The inline grammar doesn't have fenced_code_block
; so we can't inject language for code blocks
