Increment and decrement operators (`++` and `--`) are part of SystemVerilog
(IEEE1800), but not Verilog (IEEE1364).

This rule allows only binary operators with simple assigments (`foo = foo + 1`)
to encourage backwards compatibility with Verilog.

See also:
- **module_ansi_forbidden** - Useful companion rule for Verilog compatibility.
- **keyword_forbidden_always_comb** - Suggested companion rule.
- **keyword_forbidden_always_ff** - Suggested companion rule.
- **keyword_forbidden_always_latch** - Suggested companion rule.
- **keyword_forbidden_logic** - Suggested companion rule.
- **operator_self_assignment** - Suggested companion rule.

The most relevant clauses of IEEE1364-2001 are:
- 4.1 Operators
- 9.2.1 Blocking procedural assignments
- 12.1.3.2 generate-loop

The most relevant clauses of IEEE1800-2017 are:
- 10.4.1 Blocking procedural assignments
- 11.4.2 Increment and decrement operators
- 27.4 Loop generate constructs
