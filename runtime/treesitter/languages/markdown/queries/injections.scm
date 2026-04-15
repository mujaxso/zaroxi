; Markdown injections for tree-sitter-markdown-inline
; Note: The inline grammar may not support fenced code blocks
; For now, we'll keep a minimal injection setup

; HTML injection
((html_tag) @injection.content
  (#set! injection.language "html"))

; LaTeX injection if latex_block exists  
((latex_block) @injection.content
  (#set! injection.language "latex"))

; Inline code injection (limited)
; (code_span) @injection.content
;   (#set! injection.language "plaintext")
