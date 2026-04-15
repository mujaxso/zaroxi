
; Markdown highlighting query for tree-sitter-markdown-inline
; Based on common node types and capture names

; Headings
(atx_heading) @heading
(setext_heading) @heading

; Emphasis
(emphasis) @emphasis
(strong_emphasis) @strong

; Links
(link) @link
(link_text) @link
(link_destination) @string
(link_title) @string

; Code
(inline_code_span) @inline_code
(code_fence) @code_fence
(code_span) @inline_code
(fenced_code_block) @code_fence

; Block quotes
(block_quote) @block_quote

; Lists
(list) @list
(list_item) @list
(task_list_marker) @operator

; Thematic break
(thematic_break) @thematic_break

; Paragraph
(paragraph) @paragraph

; Images
(image) @link

; References
(reference_link) @link
(reference_definition) @link

; Footnotes
(footnote_reference) @link
(footnote_definition) @link

; Strikethrough
(strikethrough) @emphasis

; Escape sequences
(escape_sequence) @string

; Line breaks
(hard_line_break) @operator
(soft_line_break) @paragraph

; Tables
(table) @table
(table_header) @heading
(table_row) @table
(table_cell) @paragraph

; HTML
(html_block) @html
(html_inline) @html

; URLs and emails
(url) @string
(email) @string

; Text content
(text) @plain
