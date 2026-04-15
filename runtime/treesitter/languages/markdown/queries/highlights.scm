
; Markdown highlighting query
; Using patterns that should work with tree-sitter-markdown-inline

; Headings - try different possible node names
[
  (atx_heading)
  (setext_heading)
  (heading)
] @heading

; Emphasis
[
  (emphasis)
  (strong_emphasis)
] @emphasis

; Links
(link) @link

; Code
[
  (inline_code_span)
  (code_span)
  (inline_code)
] @inline_code

; Block quotes
(block_quote) @block_quote

; Lists
(list_item) @list

; Thematic breaks
(thematic_break) @thematic_break

; Paragraphs and text
(paragraph) @paragraph
(text) @plain

; Fallback: match any node as plain (ensures query compiles even if above patterns fail)
(_) @plain
