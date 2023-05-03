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
- **loop_statement_in_always_comb** - Useful companion rule.
- **loop_statement_in_always_ff** - Useful companion rule.
- **loop_statement_in_always_latch** - Useful companion rule.
- **sequential_block_in_always_comb** - Similar rule, different purpose.
- **sequential_block_in_always_ff** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:
- 4.6 Determinisim
- 9.2.2.3 Latched logic always_latch procedure
- 9.3.1 Sequential blocks
- 9.4.2 Event control
- 12.4 Conditional if-else statement
- 12.5 Case statement
- 12.7 Loop statements
