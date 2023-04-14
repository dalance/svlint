The consequences/purposes of this rule are perhaps subtle, particulaly in how
it works with companion rules **default_nettype_none**, **explicit_case_default**,
**explicit_if_else**, **style_indent**, and a guideline to avoid `for` within
`always_ff`.

In conjunction with these companion rules and guidelines, a nice consequence is
that editing code after the fact is "safe", i.e. not error prone.
Without `begin`/`end` adding another statement to a single-statement conditional
block may be error prone.
This is why coding styles for C-style languages often forbid writing
`if (a) foo;`, instead requiring `if (a) { foo; }` - because it's easy to forget
to add braces with an additional statement like `if (a) { foo; bar; }`.
While a simple rule is to require the use of `begin` and `end` (or `{` and `}`),
this introduces visual noise.
The goal is to guard programmers from making a simple and easy mistake.
This rule, in conjunction with the companion rules, achieves the same goal using
a different approach, in addition to providing other nice properties.

With a sequential block (marked by `begin` and `end`) you can assign to multiple
signals in a leaf conditon which can easily result in difficult-to-comprehend
logic, e.g.:
```systemverilog
always_ff @(posedge clk) begin
  if (cond) begin
    foo_q <= foo_d;       // Block was originally written for foo.
    bar_q <= red_d;       // This was added later.
  end
  bar_q <= blue_d;        // What happens to bar_q?
end
```
By forbidding sequential blocks, you enforce that exactly signal is assigned to
per leaf condition.
A nice consequence is that exactly one signal is updated on each evaluation of
the `always_ff` block.
IEEE1800-2017 specifies that if a signal is assigned to in an `always_ff` block,
then it shall not be assigned to by any other block (compile error).

An example with multiple signals in the `always_ff` is a ping-pong buffer (AKA
shunt buffer, storage of a 2-entry fifo).
Due to the construction, you can be sure that you never update both entries at
the same time, except when that is clearly explicit.
```systemverilog
  // Enforced exclusive updates, with reset and clockgate.
  always_ff @(posedge clk)
    if (rst)
      {ping_q, pong_q} <= '0; // Assignment to multiple signals is explicit.
    else if (clkgate)
      if (foo) ping_q <= foo;
      else     pong_q <= foo;
    else // Optional explicit else.
      {ping_q, pong_q} <= {ping_q, pong_q};
```

Another example with multiple signals is an address decoder.
Due to the construction, you can be sure that you aren't accidentally updating
multiple registers on a write to one address.
```systemverilog
  // Enforced exclusivity of address decode.
  always_ff @(posedge clk)
    if (write)
      case (addr)
        123:        red_q   <= foo;
        456:        blue_q  <= foo;
        789:        green_q <= foo;
        default:    black_q <= foo; // Optional explicit default.
      endcase
```

When you don't need those exclusivity properties, only one signal should be
updated per `always_ff`.
That ensures that the code doesn't get too deep/complex/unintuitive and
drawing a logical diagram is straightforward.
This is the expected form for most signals.
```systemverilog
  always_ff @(posedge clk)
    if (rst)          ctrl_q <= '0;
    else if (clkgate) ctrl_q <= ctrl_d;
    else              ctrl_q <= ctrl_q; // Optional explicit else.
```

See also:
- **default_nettype_none** - Useful companion rule.
- **explicit_case_default** - Useful companion rule.
- **explicit_if_else** - Useful companion rule.
- **style_indent** - Useful companion rule.
- **sequential_block_in_always_comb** - Similar rule, different purpose.
- **sequential_block_in_always_latch** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:
- 4.6 Determinisim
- 9.2.2.4 Sequential logic always_ff procedure
- 9.3.1 Sequential blocks
- 9.4.2 Event control
- 12.4 Conditional if-else statement
- 12.5 Case statement
- 12.7 Loop statements
