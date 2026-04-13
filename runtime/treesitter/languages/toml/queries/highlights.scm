; TOML highlight queries for Tree‑sitter
; Based on tree-sitter-toml grammar version 0.20

(comment) @comment

(string) @string
(escape_sequence) @string.escape

(integer) @number
(float) @number

(boolean) @constant.builtin

(date_time) @constant.builtin

; Tables
(table_header (identifier) @type)
(table (identifier) @type)
(table_array_element (identifier) @type)

; Key-value pairs
(pair (key) @property)
(pair (bare_key) @property)
(pair (dotted_key (identifier) @property))
(pair (quoted_key) @property)

; Arrays
(array) @operator
(array (value) @none)

; Inline tables
(inline_table) @operator
(inline_table_pair (key) @property)

; Punctuation
"=" @operator
"[" @operator
"]" @operator
"{" @operator
"}" @operator
"," @operator
"." @operator
"+" @operator
"-" @operator
