The keywords `always_comb`, `always_ff`, and `always_latch` were added to
SystemVerilog (IEEE1800) to require extra safety checks at compile time.
Verilog (IEEE1364) only has `always`, which can describe equivalent behaviour
but without the compile-time checks.
This rule requires `always @*` to be used instead of `always_comb` for
backwards compatibility with Verilog.

See also:
- **keyword_forbidden_always_ff** - Suggested companion rule.
- **keyword_forbidden_always_latch** - Suggested companion rule.
- **module_ansi_forbidden** - Useful companion rule for Verilog compatibility.
- **operator_self_assignment** - Suggested companion rule.

The most relevant clauses of IEEE1364-2001 are:
- 9.9 Structured procedures

The most relevant clauses of IEEE1800-2017 are:
- 9.2 Structured procedures
