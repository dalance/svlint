This rule checks the leading whitespace immediately following any arithmetic operator:
`+`
, `-`
, `*`
, `/`
, `%`
, and `**`.
Uses of these operators may have a single space between the
operator's symbol and the leading symbol or identifier, e.g.
`a + b`,
, or `a+b`.

In relation to Annex A of IEEE1800-2017, this rule applies to the specific
variants of `binary_operator` specified in Table 11-3.

See also:

- **style_operator_arithmetic** - Suggested companion rule. This is the rule for trailing whitespace.
