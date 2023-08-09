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
Uses of these operators must have single space between the
operator's symbol and the leading symbol or identifier, e.g.
`1 << 5`,
, or `8'hAA | 8'h55`.

In relation to Annex A of IEEE1800-2017, this rule applies to specific variants
of `binary_operator` and `binary_module_path_operator`.

See also:

- **style_operator_integer** - Suggested companion rule. This is the rule for trailing whitespace.
- **style_operator_arithmetic_leading_space** - Suggested companion rule.
- **style_operator_boolean_leading_space** - Suggested companion rule.
