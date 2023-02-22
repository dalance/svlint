This rule has two purposes:
1. Prevent mismatches between simulation and synthesis.
2. Avoid unnecessarily restricting the simulator's scheduler.

An `always_comb` block is scheduled for execution whenever any of the RHS
variables (or nets) change value, which can lead to unnecessary sequential
dependencies.
For example, the following block is requires that the "expensive" (in terms
of CPU time) function must be called to update `a` whenever `z` changes value,
in addition to whenever `y` changes value.
```systemverilog
always_comb begin
  a = expensive(y);
  b = z;
end
```

The above example can be reformed to allow the simulator more flexibility in
how it schedules processes.
Logical equivalence is maintained, and a synthesis tool will interpret these
examples equivalently.
Note that continuous assignment (using `assign`) is not sensitive to changes in
`y` because functions are not transparent.
```systemverilog
always_comb a = expensive(y);
assign b = z;
```

This rule is intended for synthesisable code only, not testbench code.
Testbenches often necessarily rely on sequential dependencies, but a synthesis
tool for digital synchronous logic will produce a netlist without sequential
dependencies.
That can lead to a mismatch between simulation and synthesis.

See also:
- **style_indent** - Useful companion rule.
- **sequential_block_in_always_ff** - Similar rule, different purpose.
- **sequential_block_in_always_latch** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:
- 4.6 Determinisim
- 9.2.2.2 Combinational logic always_comb procedure
- 9.3.1 Sequential blocks
- 10.3 Continuous assignments
- 10.4 Procedural assignments
