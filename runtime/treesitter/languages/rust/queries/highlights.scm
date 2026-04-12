; Rust highlight queries for Tree‑sitter
; This is a minimal set; you can expand it with more captures.

(
  (line_comment) @comment
  (block_comment) @comment
)

(
  (string_literal) @string
  (raw_string_literal) @string
)

(
  (char_literal) @string
)

(
  (identifier) @variable
)

(
  (type_identifier) @type
)

(
  "fn" @keyword
  "let" @keyword
  "mut" @keyword
  "pub" @keyword
  "use" @keyword
  "mod" @keyword
  "struct" @keyword
  "enum" @keyword
  "impl" @keyword
  "trait" @keyword
  "where" @keyword
  "match" @keyword
  "if" @keyword
  "else" @keyword
  "for" @keyword
  "in" @keyword
  "while" @keyword
  "loop" @keyword
  "return" @keyword
  "break" @keyword
  "continue" @keyword
  "self" @keyword
  "Self" @keyword
  "true" @constant
  "false" @constant
  "as" @keyword
  "ref" @keyword
  "move" @keyword
)

(
  (integer_literal) @number
  (float_literal) @number
)

(
  (call_expression
    function: (identifier) @function.call)
)

(
  (field_expression
    field: (field_identifier) @property)
)

; Function definitions
(
  (function_item
    name: (identifier) @function)
)

; Struct definitions
(
  (struct_item
    name: (type_identifier) @type)
)

; Enum definitions
(
  (enum_item
    name: (type_identifier) @type)
)

; Module declarations
(
  (mod_item
    name: (identifier) @namespace)
)
