This rule checks the whitespace immediately following any binary operator whose
operation returns a boolean:
`==`
, `!=`
, `===`
, `!==`
, `==?`
, `!=?`
, `&&`
, `||`
, `<`
, `<=`
, `>`
, `>=`
, `->`
, and `<->`.
Uses of these operators must have a single space between the operator's symbol
and the following symbol or identifier, e.g.
`a && b`,
, `c !== d`
, or `0 < 5`.

In relation to Annex A of IEEE1800-2017, this rule applies to specific variants
of `binary_operator` and `binary_module_path_operator`.

See also:
  - **style_operator_arithmetic** - Suggested companion rule.
  - **style_operator_integer** - Suggested companion rule.
  - **style_operator_unary** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.
