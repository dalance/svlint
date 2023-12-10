Continuous assignment, e.g. `assign x = y;` outside of any `always` process,
continuously drives the LHS and changes with any change on the RHS.
The same keyword `assign` has different meaning when used within an `always`
process (or `always_ff`, `always_comb`, `initial`, etc.) where it can be used
to override procedural assignments.
Using this construct in a procedural block (`always*`) which is only triggered
on changes to signals in the sensitivity list may not be synthesizable.

This SystemVerilog language feature is being considered for deprecation, as
noted in IEEE1800-2017 Annex C, because it is easily abused and difficult to
implement while not providing additional capability.
Users are strongly encouraged to migrate their cod to use one of the alternate
methods of procedural or continuous assignments.

See also:
- **non_blocking_assignment_in_always_comb** - Useful companion rule.
- **blocking_assignment_in_always_ff** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 10.6.1 The assign and deassign procedural statements
- Annex C.4 Constructs identified for deprecation
