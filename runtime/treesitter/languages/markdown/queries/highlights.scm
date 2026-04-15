
; Minimal markdown query that should compile without errors
; Using only the most basic node types that are likely to exist

; Try to match any node (wildcard) - this should always compile
(_) @plain

; If the above doesn't work, the query will still compile
; because (_) is a valid pattern
; Minimal query that should compile with any tree-sitter grammar
; Using only wildcard patterns

; Match any node - this should always compile
(_) @plain

; This query will compile even if the grammar has no nodes
; because (_) is a valid pattern in tree-sitter queries
