
; Minimal markdown highlighting query
; Starting with basic nodes to avoid compilation errors
; We can add more patterns once we confirm which node types exist

; Basic text nodes
(text) @plain

; Paragraphs
(paragraph) @paragraph

; If these node types exist, we'll get some highlighting
; Otherwise, the query will still compile successfully
