Simulator event ordering between blocking and non-blocking assignments
is undefined, so observed behavior is simulator-dependent.
This rule forbids the use of non-blocking assigments (using the `<=` operator)
in `always_comb` blocks.
Instead, use the blocking assignment operator `=`.

An excellent paper detailing the semantics of Verilog blocking and non-blocking
assignments is written by Clifford E Cummings and presented at SNUG-2000,
"Nonblocking Assignments in Verilog Synthesis, Coding Styles that Kill".

See also:
- **blocking_assignment_in_always_ff** - Useful companion rule.
- **blocking_assignment_in_always_latch** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 4.9.3 Blocking assignment
- 4.9.4 Non-blocking assignment
- 9.2.2.2 Combinational logic `always_comb` procedure
- 9.4.2 Event control
- 10.4.1 Blocking procedural assignments
- 10.4.2 Nonblocking procedural assignments
