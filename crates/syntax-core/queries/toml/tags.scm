; TOML tags queries for symbol navigation

(table_header (dotted_key (bare_key) @definition.type))
(table_header (dotted_key (quoted_key) @definition.type))
(table_array_header (dotted_key (bare_key) @definition.type))
(table_array_header (dotted_key (quoted_key) @definition.type))

; Key-value pairs can be considered as definitions
(key_value (bare_key) @definition.field)
(key_value (quoted_key) @definition.field)
(key_value (dotted_key (bare_key) @definition.field))
(key_value (dotted_key (quoted_key) @definition.field))
