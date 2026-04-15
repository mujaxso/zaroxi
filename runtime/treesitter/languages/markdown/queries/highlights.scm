
; Markdown highlighting for tree-sitter-markdown with correct capture names
(atx_heading) @heading
(setext_heading) @heading
(emphasis) @emphasis
(strong_emphasis) @strong
(link) @link
(inline_code_span) @inline_code
(code_fence) @code_fence
(block_quote) @block_quote
(list) @list
(thematic_break) @thematic_break
(paragraph) @paragraph
(fenced_code_block) @code_fence
(code_span) @inline_code
(image) @link
(reference_link) @link
(reference_definition) @link
(footnote_reference) @link
(footnote_definition) @link
(task_list_marker) @operator
(strikethrough) @emphasis
(escape_sequence) @string
(hard_line_break) @operator
(soft_line_break) @paragraph
(table) @table
(table_header) @heading
(table_row) @table
(table_cell) @paragraph
(html_block) @html
(html_inline) @html

; Additional captures that might be present
(heading_content) @heading
(list_marker) @operator
(link_label) @link
(link_title) @string
(url) @string
(email) @string
