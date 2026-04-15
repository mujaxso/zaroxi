; Markdown highlighting query for tree-sitter-markdown-inline
; Based on actual node types from debug output
; Using only nodes that exist in the inline grammar

; Escape sequences
(backslash_escape) @escape

; Emphasis
(emphasis) @emphasis
(strong_emphasis) @strong_emphasis

; Code
(code_span) @inline_code

; Links
(link_text) @link.text
(link_destination) @link.destination
(link_title) @link.title
(shortcut_link) @link
(full_reference_link) @link
(collapsed_reference_link) @link
(inline_link) @link

; Images
(image) @image
(image_description) @image.description

; HTML
(html_tag) @html

; Line breaks
(hard_line_break) @line_break

; Strikethrough
(strikethrough) @strikethrough

; Autolinks
(uri_autolink) @link.autolink
(email_autolink) @link.autolink

; Fallback for everything else
(_) @plain
