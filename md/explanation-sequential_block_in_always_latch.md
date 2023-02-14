The explanation of **sequential_block_in_always_ff**, and much of the explanation
of **sequential_block_in_always_comb**, also applies to this rule.
Main points are that avoiding `begin`/`end` helps protect the programmer against
simple mistakes, provides exclusivity properties by construction, and avoids
restricting simulator scheduling decisions.

See also:

- **default_nettype_none** - Useful companion rule.
- **explicit_case_default** - Useful companion rule.
- **explicit_if_else** - Useful companion rule.
- **style_indent** - Useful companion rule.
- **sequential_block_in_always_comb** - Similar rule, different purpose.
- **sequential_block_in_always_ff** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:

- 4.6 Determinisim
- 49.2.2.3 Latched logic always_latch procedure
- 49.3.1 Sequential blocks
- 49.4.2 Event control
- 412.4 Conditional if-else statement
- 412.5 Case statement
- 412.7 Loop statements
