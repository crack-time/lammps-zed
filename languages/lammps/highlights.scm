; Comments
(comment) @comment

; Strings
(single_quote_string) @string
(double_quote_string) @string

; Numbers
(number) @number

; Variables
(variable) @variable

; Operators
(operator) @operator

; Brackets
"(" @punctuation.bracket
")" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

; Command names — highlight as functions
(command_name) @function

; Command names that act as keywords
; (match? @function "^(if|then|else|elif|jump|loop|next|quit|shell|print|include|label|variable|clear)$")
