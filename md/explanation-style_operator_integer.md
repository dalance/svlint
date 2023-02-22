This rule checks the whitespace immediately following any binary operator whose
operation returns an integer (except arithmetic operators):
`&`
, `|`
, `^`
, `^~`
, `~^`
, `>>`
, `<<`
, `>>>`
, and `<<<`.
Uses of these operators must have single space or a newline between the
operator's symbol and the following symbol or identifier, e.g.
`1 << 5`,
, or `8'hAA | 8'h55`.

In relation to Annex A of IEEE1800-2017, this rule applies to specific variants
of `binary_operator` and `binary_module_path_operator`.

See also:
- **style_operator_arithmetic** - Suggested companion rule.
- **style_operator_boolean** - Suggested companion rule.
- **style_operator_unary** - Suggested companion rule.
