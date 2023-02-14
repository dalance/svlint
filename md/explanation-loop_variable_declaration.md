A loop variable may be declared either inside the loop, e.g.
`for (int i = 0; i < 5; i++)`, or outside the loop, e.g.
`int i; ... for (i = 0; i < 5; i++)`.
This rule mandates that the scope of a loop variable, e.g. `i`, is minimized to
avoid a common class of coding mistake where `i` is erroneously used outside
the loop.

See also:

- **function_with_automatic** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:

- 12.7 Loop statements
