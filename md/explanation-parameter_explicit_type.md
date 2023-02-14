The type of a parameter is more fundmental to express intent than its value.
By analogy, asking a shopkeeper for "5 oranges" is more likely to be correctly
understood than simply asking for "5" without clarification.
This rule requires that authors consider and specify the type of each
module `parameter` port.
Explicit types help readers, particularly large-scale integrators, to
understand exactly what values are expected, thus reducing the effort they need
to expend reading how the parameter is used.

Without an explicit type, a module parameter will take a type compatible with
its default assignment, or a type compatible with any override values.
Implict types can thereby introduce discrepencies between what the author
intends and how tools interpret the code.
For example, interactions between the default datatype `logic`, constant
functions, and case expressions can result in mismatches between simulation and
synthesis.
A detailed investigation into the semantics of implicit vs explicit types
on SystemVerilog `parameter` and `localparam`s can be found in a tutorial
paper here:
<https://github.com/DaveMcEwan/dmpvl/tree/master/prs/paper/ParameterDatatypes>

See also:

- **localparam_explicit_type** - Useful companion rule.
- **localparam_type_twostate** - Useful companion rule.
- **parameter_type_twostate** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:

- 6.3 Value set
- 6.11 Integer data types
- 6.20.2 Value parameters
- 23.2.3 Parameterized modules
