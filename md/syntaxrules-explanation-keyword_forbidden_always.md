In Verilog (IEEE1364), there are two language constructs which can be used to
model combinatorial logic:
1. Continuous assignment to `wire` signals is specified with the `assign`
  keyword.
2. `reg` signals are assigned to with an `always` block, which is evaluated
  whenever anything in the sensitivity list changes value.

The `always` keyword can also be used for modelling sequential logic by
including the edge of a signal in the sensitivity list.

The semantics of these keywords in SystemVerilog are compatible with Verilog,
but additional keywords (`always_comb`, `always_ff`, and `always_latch`) should
be used to clarify intent of digital designs.
The `always_*` keywords have slightly different semantics which are beneficial
for synthesizable designs:
1. `always_*` processes require compiler checks that any signals driven on the
  LHS are not driven by any other process, i.e. `always_*` cannot infer
  multi-driven or tri-state logic.
2. `always_comb` processes require a compiler check that the process does not
  infer state.
3. `always_ff` processes require a compiler check that the process does infer
  state.

This rule forbids the use of the general-purpose `always` keyword, thus forcing
authors of synthesizable design code to clarify their intent.
In verification code to be used in simulation only, a general-purpose `always`
process is a valid and useful way of scheduling events.
Therefore, this rule is intended only for synthesizable design code, not for
testbench code.

The alternative rule **general_always_no_edge** has similar reasoning but is
slightly relaxed, requiring that `always` blocks have an explicit sensitivity
list including an edge.
It is possible to construct a full-featured testbench where all `always` blocks
meet that requriment.
Therefore, it is appropriate to use **keyword_forbidden_always** on
synthesizable design code, but on verification code use
**general_always_no_edge** instead.

See also:
- **general_always_no_edge** - Alternative rule.
- **general_always_level_sensitive** - Alternative rule.
- **sequential_block_in_always_comb** - Useful companion rule.
- **sequential_block_in_always_if** - Useful companion rule.
- **sequential_block_in_always_latch** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 9.2.2 Always procedures
- 9.5 Process execution threads
