This rule checks the whitespace immediately following any arithmetic operator:
`+`
, `-`
, `*`
, `/`
, `%`
, and `**`.
Uses of these operators may have a single space or newline between the
operator's symbol and the following symbol or identifier, e.g.
`a + b`,
, or `a+b`.

In relation to Annex A of IEEE1800-2017, this rule applies to the specific
variants of `binary_operator` specified in Table 11-3.

See also:

- **style_operator_boolean** - Suggested companion rule.
- **style_operator_integer** - Suggested companion rule.
- **style_operator_unary** - Suggested companion rule.
- **style_operator_arithmetic_leading_space** - Suggested companion rule. This is the rule for leading whitespace.
