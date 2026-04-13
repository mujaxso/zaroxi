; TOML syntax highlighting queries for Tree-sitter

; Comments
(comment) @comment

; Basic values
(string) @string
(integer) @number
(float) @number
(boolean) @constant.builtin
(offset_date_time) @string.special
(local_date_time) @string.special
(local_date) @string.special
(local_time) @string.special

; Tables
(table_header (dotted_key (bare_key) @type))
(table_header (dotted_key (quoted_key) @type))
(table_array_header (dotted_key (bare_key) @type))
(table_array_header (dotted_key (quoted_key) @type))

; Keys
(bare_key) @variable
(quoted_key) @variable

; Key-value pairs
(key_value (bare_key) @variable)
(key_value (quoted_key) @variable)
(key_value (dotted_key (bare_key) @variable))
(key_value (dotted_key (quoted_key) @variable))

; Arrays
(array (string) @string)
(array (integer) @number)
(array (float) @number)
(array (boolean) @constant.builtin)
(array (offset_date_time) @string.special)
(array (local_date_time) @string.special)
(array (local_date) @string.special)
(array (local_time) @string.special)

; Punctuation
"[" @punctuation.bracket
"]" @punctuation.bracket
"[[" @punctuation.bracket
"]]" @punctuation.bracket
"." @punctuation.delimiter
"=" @operator
