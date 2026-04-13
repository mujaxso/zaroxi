; TOML locals queries for variable tracking
; TOML doesn't have scoped variables in the traditional sense,
; but we can track table definitions

(table_header (dotted_key (bare_key) @definition.type))
(table_header (dotted_key (quoted_key) @definition.type))
(table_array_header (dotted_key (bare_key) @definition.type))
(table_array_header (dotted_key (quoted_key) @definition.type))

; Reference to tables in dotted keys
(dotted_key (bare_key) @reference)
(dotted_key (quoted_key) @reference)
