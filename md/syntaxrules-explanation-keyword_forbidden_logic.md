The datatype `logic` was added to SystemVerilog (IEEE1800) to clarify
designer's intent, mostly replacing `wire` and fully replacing `reg`.
Verilog (IEEE1364) only has the `reg` bit-vector variable (and the various type
of nets).
This rule forbids `logic` for backwards compatibility with Verilog.

See also:
- **keyword_forbidden_always_comb** - Suggested companion rule.
- **keyword_forbidden_always_ff** - Suggested companion rule.
- **keyword_forbidden_always_latch** - Suggested companion rule.
- **module_ansi_forbidden** - Useful companion rule for Verilog compatibility.
- **operator_incdec** - Suggested companion rule.
- **operator_self_assignment** - Suggested companion rule.

The most relevant clauses of IEEE1364-2001 are:
- 3.2 Nets and variables
- 3.3 Vectors
- 3.7 Nets types
- 3.8 regs

The most relevant clauses of IEEE1800-2017 are:
- 6.5 Nets and variables
- 6.5 Vector declarations
- 6.11 Integer data types
