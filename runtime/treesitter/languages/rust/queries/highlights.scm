; Rust highlight queries for Tree‑sitter (compatible with tree‑sitter‑rust 0.20)

(block_comment) @comment
(line_comment) @comment

(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

(integer_literal) @number
(float_literal) @number

(identifier) @variable
(type_identifier) @type

(call_expression
  function: (identifier) @function.call)

(function_item
  name: (identifier) @function)

(struct_item
  name: (type_identifier) @type)

(enum_item
  name: (type_identifier) @type)

[
  "fn"
  "let"
  "mut"
  "pub"
  "use"
  "mod"
  "struct"
  "enum"
  "impl"
  "trait"
  "where"
  "match"
  "if"
  "else"
  "for"
  "in"
  "while"
  "loop"
  "return"
  "break"
  "continue"
  "self"
  "Self"
  "as"
  "ref"
  "move"
] @keyword

[
  "true"
  "false"
] @constant
