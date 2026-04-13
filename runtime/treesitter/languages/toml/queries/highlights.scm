; TOML highlight queries for Tree‑sitter
; Based on tree-sitter-toml grammar version 0.20
; Updated to match actual node types from the grammar

; Comments
(comment) @comment

; Strings and escapes
(string) @string
(escape_sequence) @string.escape

; Numbers
(integer) @number
(float) @number

; Constants
(boolean) @constant.builtin
; Date/time values - check if these node types exist in the grammar
; (date_time) @constant.builtin
; (local_date) @constant.builtin
; (local_time) @constant.builtin
; (local_date_time) @constant.builtin

; Tables
(table (identifier) @type)
(array_table (identifier) @type)

; Keys in key-value pairs
(pair (key) @property)
(pair (bare_key) @property)
(pair (quoted_key) @property)
(pair (dotted_key (identifier) @property))

; Array and inline table delimiters
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

; Operators
"=" @operator
"," @punctuation.delimiter
"." @punctuation.delimiter
"+" @operator
"-" @operator
