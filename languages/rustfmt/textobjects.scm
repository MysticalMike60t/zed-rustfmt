; [1] Zed Extensions, TOML, GitHub, Apr. 2026. [Online]. Available: https://github.com/zed-extensions/toml

(comment)+ @comment.around
(table "[" (_) "]"
    (_)* @class.inside) @class.around

(table_array_element "[[" (_) "]]"
    (_)* @class.inside) @class.around
