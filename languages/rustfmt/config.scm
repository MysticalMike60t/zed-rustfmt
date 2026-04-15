; Valid Rusfmt "rustfmt.toml" Config Keys
(pair
    (bare_key) @keyword
    (#match? @keyword "^(array_width)$"))

; Invalid Rusfmt "rustfmt.toml" Config Keys
(pair
    (bare_key) @error
    (#not-match? @error "^(array_width)$"))
