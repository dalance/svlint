Self-assignment operators (`+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`,
`<<=`, `>>=`, `<<<=`, and `>>>=`) are part of SystemVerilog (IEEE1800), but not
Verilog (IEEE1364).

This rule allows only simple assigment (using `=`) to encourage backwards
compatibility with Verilog.

See also:
- **module_ansi_forbidden** - Useful companion rule for Verilog compatibility.
- **keyword_forbidden_always_comb** - Suggested companion rule.
- **keyword_forbidden_always_ff** - Suggested companion rule.
- **keyword_forbidden_always_latch** - Suggested companion rule.

The most relevant clauses of IEEE1364-2001 are:
- 4.1 Operators
- 9.2.1 Blocking procedural assignments

The most relevant clauses of IEEE1800-2017 are:
- 10.4.1 Blocking procedural assignments
- 11.4.1 Assignment operators
