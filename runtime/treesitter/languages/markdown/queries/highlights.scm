; Markdown highlighting query for tree-sitter-markdown-inline
; Enhanced with more specific inline patterns for better highlighting
; Using only valid node types (no underscore-prefixed nodes)

; ====== Escape sequences ======
(backslash_escape) @escape

; ====== Emphasis and strong emphasis ======
(emphasis) @emphasis
(strong_emphasis) @strong_emphasis

; Emphasis delimiters - use emphasis_delimiter which is valid
(emphasis_delimiter) @emphasis.marker

; ====== Code spans ======
(code_span) @inline_code
(code_span_delimiter) @inline_code.delimiter

; ====== Links ======
(link_text) @link.text
(link_destination) @link.destination
(link_title) @link.title
(link_label) @link.label

; Different link types
(shortcut_link) @link
(full_reference_link) @link
(collapsed_reference_link) @link
(inline_link) @link

; Link brackets and parentheses
"[" @link.bracket
"]" @link.bracket
"(" @link.paren
")" @link.paren

; ====== Images ======
(image) @image
(image_description) @image.description

; Image brackets and exclamation mark
"![" @image.marker

; ====== HTML ======
(html_tag) @html

; ====== Line breaks ======
(hard_line_break) @line_break

; ====== Strikethrough ======
(strikethrough) @strikethrough

; ====== Autolinks ======
(uri_autolink) @link.autolink
(email_autolink) @link.autolink

; ====== Entity references ======
(entity_reference) @escape
(numeric_character_reference) @escape

; ====== LaTeX ======
(latex_block) @latex
(latex_span_delimiter) @latex.delimiter

; ====== Inline content ======
; Note: _word, _digits, etc. start with underscores and can't be captured
; We'll capture the actual content nodes instead

; ====== Punctuation ======
; Capture specific punctuation that's part of markdown syntax
"[" @punctuation
"]" @punctuation
"(" @punctuation
")" @punctuation
"!" @punctuation
"*" @punctuation
"_" @punctuation
"`" @punctuation
"~" @punctuation
"\\" @punctuation

; ====== Fallback for everything else ======
(_) @plain
