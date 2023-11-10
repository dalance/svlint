This rule is specific to code which must be compatible with Verilog, not
only SystemVerilog.

In Verilog (IEEE1364), there are two language constructs which can be used to
model combinatorial logic:
1. Continuous assignment to `wire` signals is specified with the `assign`
  keyword.
2. `reg` signals are assigned to with an `always` block, which is evaluated
  whenever anything in the sensitivity list changes value.

To ensure that the process correctly sensitive to changes on all driving
signals, `always @*` should be used instead of providing an explicit
sensitivity list like `always @(a or b or c)`.
The `always` keyword can also be used for modelling sequential logic by
including the edge of a signal in the sensitivity list.
Providing an explicit sensitivity list is prone to two mistakes:
1. Forgetting to add a driver to the list, e.g. `always @(b) a = b + c;`
   instead of `always @(b or c) a = b + c;`.
2. Forgetting to add and edge specifier, e.g. `always @(clk) q <= d;` instead
   of `always @(posedge clk) q <= d;`.
   That makes the process level-sensitive, instead of the edge-sensitive.

This rule requires that general-purpose `always` blocks with an explicit
sensitivity list which include at least one edge.
Combinational logic should use the Kleen-star notation,
e.g. `always @* a = b + c;`

See also:
- **keyword_forbidden_always** - Related rule forbidding general-purpose
  `always`, only applicable for SystemVerilog code.
- **general_always_no_edge** - Related rule forbidding purely combinational
  logic in `always` processes.
  While this is straightforward to use with SystemVerilog, this might be overly
  restrictive for Verilog because all combinational variables must be driven
  with `assign`.

The most relevant clauses of IEEE1800-2017 are:
- 9.2.2 Always procedures
- 9.5 Process execution threads
