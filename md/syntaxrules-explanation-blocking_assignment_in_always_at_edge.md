Simulator event ordering between blocking and non-blocking assignments
is undefined, so observed behavior is simulator-dependent.
Edge-sensitive (usually clocked) processes like, `always @(posedge clk)` should
only contain non-blocking assignments in order for sampling and variable
evaluation to operate in a defined order, e.g. `q <= d;`, not `q = d;`.

For SystemVerilog (IEEE1800) code, the keyword `always_ff` (or `always_latch`)
should be used instead of the general purpose `always` to take advantage of
extra compile-time checks.
For code which must be compatible with Verilog (IEEE1364), `always` is the only
option.
Therefore, this rule `reg` assignments to be compatible with Verilog like this
(in conjunction with **non_blocking_assignment_in_always_no_edge**):
```verilog
always @(posedge clk) q <= d;       // Clocked to reg (flip-flop)
always @* a = b + c;                // Combinational to reg (logic gates)
assign d = e + f;                   // Combinational to wire (logic gates)
```

See also:
- **non_blocking_assignment_in_always_no_edge** - Useful companion rule.
- **blocking_assignment_in_always_ff** - Similar rule, suggested as alternative
  for SystemVerilog code, but not Verilog.
- **blocking_assignment_in_always_latch** - Useful companion rule for
  SystemVerilog, but not Verilog.
- **non_blocking_assignment_in_always_comb** - Useful companion rule for
  SystemVerilog, but not Verilog.

The most relevant clauses of IEEE1800-2017 are:
- 4.9.3 Blocking assignment
- 4.9.4 Non-blocking assignment
- 9.4.2 Event control
- 10.4.1 Blocking procedural assignments
- 10.4.2 Nonblocking procedural assignments
- 16.5.1 Sampling
