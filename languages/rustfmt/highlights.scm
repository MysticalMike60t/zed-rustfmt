; [1] Zed Extensions, TOML, GitHub, Apr. 2026. [Online]. Available: https://github.com/zed-extensions/toml

; Properties
;-----------

(bare_key) @property
(quoted_key) @property

; Literals
;---------

(boolean) @constant
(comment) @comment
(integer) @number
(float) @number
(string) @string
(escape_sequence) @string.escape
(offset_date_time) @string.special
(local_date_time) @string.special
(local_date) @string.special
(local_time) @string.special

; Punctuation
;------------

[
  "."
  ","
] @punctuation.delimiter

"=" @operator

[
  "["
  "]"
  "[["
  "]]"
  "{"
  "}"
]  @punctuation.bracket

; Specifics
;----------
; Valid Rusfmt "rustfmt.toml" Config Keys
(pair
    (bare_key) @keyword
    (#match? @keyword "^(array_width)$"))

; Invalid Rusfmt "rustfmt.toml" Config Keys
(pair
    (bare_key) @error
    (#not-match? @error "^(array_width)$"))
