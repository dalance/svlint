IEEE1800-2017 (clause 9.2.2.2) comments that tools should *warn* if an
`always_comb` procedure infers memory.
However, simulators and synthesis tools are not required to enforce that
`always_comb` procedures only infer combinational logic.
This allows for simulators and synthesis tools to interpret these procedures
differently, which results in a mismatch between simulation and synthesis.

An incomplete case statement may be interpreted as latched logic,
e.g: `always_comb case (foo) '0: a = 5; endcase`.
Only the case where `foo == 0` is specified, to update variable `a` to the
value `5`.
When `foo` is non-zero, this example may be interpreted in at least two ways:
- `a = 'x;` - As the new value is not specified, it is unknown.
  A synthesis tool may allow node `a` to be undriven, or choose to drive
  `a` equivalently to one of the explicitly specified case expressions.
- `a = a;` - As the new value is not specified, do not change `a`.
  A synthesis tool may produce a latching circuit.

See also:
- **explicit_case_default** - Useful companion rule.
- **explicit_if_else** - Useful companion rule.
- **legacy_always** - Useful companion rule.
- **sequential_block_in_always_comb** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 9.2.2.2 Combinational logic `always_comb` procedure
- 12.5 Case statement
- 13.4 Functions
