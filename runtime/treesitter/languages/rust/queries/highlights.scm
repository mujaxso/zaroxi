					; Rust highlight queries for Tree‑sitter (compatible with tree‑sitter‑rust 0.20)

; Comments
(block_comment) @comment
(line_comment) @comment

; Strings
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

; Keywords
[
  "as"
  "async"
  "await"
  "break"
  "const"
  "continue"
  "crate"
  "dyn"
  "else"
  "enum"
  "extern"
  "false"
  "fn"
  "for"
  "if"
  "impl"
  "in"
  "let"
  "loop"
  "match"
  "mod"
  "move"
  "mut"
  "pub"
  "ref"
  "return"
  "self"
  "Self"
  "static"
  "struct"
  "super"
  "trait"
  "true"
  "type"
  "union"
  "unsafe"
  "use"
  "where"
  "while"
] @keyword

; Built-in types
[
  "bool"
  "char"
  "f32"
  "f64"
  "i8"
  "i16"
  "i32"
  "i64"
  "i128"
  "isize"
  "str"
  "u8"
  "u16"
  "u32"
  "u64"
  "u128"
  "usize"
] @type.builtin

; Function definitions
(function_item
  name: (identifier) @function)

; Function calls
(call_expression
  function: (identifier) @function.call)

; Type definitions
(type_identifier) @type
(primitive_type) @type.builtin

; Variables
(identifier) @variable

; Constants
(const_item
  name: (identifier) @constant)

; Parameters
(parameter
  pattern: (identifier) @variable.parameter)

; Attributes
(attribute_item) @attribute

; Macros
(macro_invocation
  macro: (identifier) @macro)

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
