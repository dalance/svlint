There are 4 kinds of SystemVerilog port (`inout`, `input`, `output`, and `ref`),
though `ref` is not generally used for synthesisable code.
For a new reader, unfamiliar with a large module, it is useful to be able to
distinguish at a glance between which signals are ports and internal ones.
This is especially useful for an integrator who needs to read and understand the
boundaries of many modules quickly and accurately.
To use a visual analogy, prefixing port names is like adding arrowheads to a
schematic - they're not essential, but they speed up comprehension.
This rule requires the prefix `o_` (configurable) on `output` signals.

See also:
- **prefix_inout** - Suggested companion rule.
- **prefix_input** - Suggested companion rule.
- **prefix_instance** - Suggested companion rule.
