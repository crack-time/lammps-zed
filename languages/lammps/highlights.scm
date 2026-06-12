(comment) @comment
(inline_comment) @comment

(single_quote_string) @string
(double_quote_string) @string

(number) @number

(variable) @variable

(operator) @operator

(command_name) @keyword

(line_continuation) @string.escape
"&" @string.escape

; Control flow in expressions
((word) @keyword.conditional
 (#match? @keyword.conditional "^(if|then|elif|else|while|for|return|equal)$"))

; Boolean values
((word) @boolean
 (#match? @boolean "^(on|off|true|false|yes|no)$"))

; Special constants
((word) @constant.builtin
 (#match? @constant.builtin "^(NULL|EDGE|PI|INF)$"))

; Built-in functions in expressions (math, group, region, special, feature)
((word) @function
 (#match? @function "^(sqrt|exp|ln|log|abs|sin|cos|tan|asin|acos|atan|atan2|random|normal|ceil|floor|round|ramp|stagger|logfreq|stride|vdisplace|swiggle|cwiggle|sum|min|max|ave|trap|slope|gmask|rmask|grmask|next|is_file|is_available|is_active|is_defined|count|mass|charge|xcm|vcm|fcm|bound|gyration|ke|angmom|torque|inertia|omega)$"))

"(" @punctuation.bracket
")" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
