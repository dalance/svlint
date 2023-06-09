This rule checks the whitespace immediately following any unary operator:
`++`
, `--`
, `+`
, `-`
, `!`
, `~`
, `&`
, `~&`
, `|`
, `~|`
, `^`
, `~^`
, and `^~`.
Uses of these operators must never have any whitespace between the operator's
symbol and the following symbol or identifier, e.g.
`i++`,
`!FOO`,
, `&{a, b, c}`
, or `$info("%d", -5);`.

In relation to Annex A of IEEE1800-2017, this rule applies to all variants of
`unary_operator`, `unary_module_path_operator`, and `inc_or_dec_operator`.

See also:
- **style_operator_arithmetic** - Suggested companion rule.
- **style_operator_boolean** - Suggested companion rule.
- **style_operator_integer** - Suggested companion rule.
