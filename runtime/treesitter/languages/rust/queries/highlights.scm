					; Rust highlight queries for Tree‑sitter (compatible with tree‑sitter‑rust 0.20)

; Comments
(block_comment) @comment
(line_comment) @comment

; Strings
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

; Keywords - using individual patterns instead of list
"fn" @keyword
"let" @keyword
"if" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"match" @keyword
"struct" @keyword
"enum" @keyword
"impl" @keyword
"trait" @keyword
"use" @keyword
"pub" @keyword
"mod" @keyword
"type" @keyword
"const" @keyword
"static" @keyword
"unsafe" @keyword
"return" @keyword
"break" @keyword
"continue" @keyword
"as" @keyword
"in" @keyword
"where" @keyword
"loop" @keyword
"move" @keyword
"ref" @keyword
"mut" @keyword
"self" @keyword
"Self" @keyword
"super" @keyword
"extern" @keyword
"crate" @keyword
"true" @keyword
"false" @keyword
"async" @keyword
"await" @keyword
"dyn" @keyword

; Function definitions
(function_item
  name: (identifier) @function)

; Type definitions
(type_identifier) @type

; Variables
(identifier) @variable

; Constants
(const_item
  name: (identifier) @constant)

; Operators
[
  "+" "-" "*" "/" "%"
  "=" "==" "!=" "<" "<=" ">" ">="
  "!" "&&" "||"
  "&" "|" "^" "<<" ">>"
  "+=" "-=" "*=" "/=" "%="
  "&=" "|=" "^=" "<<=" ">>="
  ".." "..=" "->" "=>"
] @operator

; Punctuation
[
  "," ";" ":" "::" "." "(" ")" "[" "]" "{" "}"
] @punctuation

; Literals
(boolean_literal) @constant.builtin
(integer_literal) @number
(float_literal) @number
