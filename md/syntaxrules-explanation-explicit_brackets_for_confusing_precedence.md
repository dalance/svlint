In SystemVerilog, like C, bitwise binary operators `&`, `|`, `^` and
`~^`/`^~` (XNOR) have *lower* precedence than the comparison operators
`>=`, `>`, `<`, `<=`, `==`, `!=`, `===`, `!==`, `==?` and `!=?`.
This leads to surprising behaviour in code like this:

```systemverilog
logic [7:0] x;
logic y;
assign y = x & 8'h0F == '0;
```

The intention here was `(x & 8'h0F) == '0` but SystemVerilog will
calculate `x & (8'h0F == '0)` which is always `1'b0`.

In modern languages like Go, Rust, and Swift, bitwise operators have
*higher* precedence than comparison operators.

This rule forbids unbracketed expressions containing a mix of comparison
and bitwise operators.
Instead you can explicitly add brackets:

```systemverilog
assign y = (x & 8'h0F) == '0;
```

The most relevant clauses of IEEE1800-2017 are:
- 11.3.2 Operator Precedence
