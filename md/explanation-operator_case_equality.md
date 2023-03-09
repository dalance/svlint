Case equality operations (using `===` or `!==` operators) include comparison
against `'z` or `'x`, so they are not generally synthesisable.
Synthesizable code should use logical or wildcard equality operations instead.

See also:
- **case_default** - Useful companion rule.
- **explicit_case_default** - Useful companion rule.
- **enum_with_type** - Useful companion rule.
- **localparam_type_twostate** - Useful companion rule.
- **parameter_type_twostate** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 11.4.5 Equality operators
- 11.4.6 Wildcard quality operators
