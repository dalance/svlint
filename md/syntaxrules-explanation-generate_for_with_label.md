A loop generate construct allows a single generate block to be instantiated
multiple times within a module, interface, program, or checker.
The selection of which generate blocks are instantiated is decided during
elaboration via evaluation of constant expressions.
Generate blocks introduce hierarchy within a module, whether they are named or
unnamed.
Unnamed generate blocks are assigned a name, e.g. `genblk5`, which other tools
can use and depend on.
For example, to find a specific DFF in a netlist you could use a hierarchical
path like `top.genblk2[3].u_cpu.genblk5.foo_q`.
The naming scheme for unnamed generated blocks is defined in
IEEE1800-2017 clause 27.6.

These implicit names are not intuitive for human readers, so this rule is
designed to check three things:
1. The generate block uses `begin`/`end` delimiters.
2. The generate block has been given a label, e.g. `begin: mylabel`.
3. The label has an appropriate prefix, e.g. `begin: l_mylabel` starts with
  the string `l_`.

The prefix is useful to when reading hierarchical paths to distinguish between
module/interface instances and generate blocks.
For example, `top.l_cpu_array[3].u_cpu.l_debugger.foo_q` provides the reader
with more useful information than `top.genblk2[3].u_cpu.genblk5.foo_q`.

See also:
- **generate_case_with_label** - Similar reasoning, useful companion rule.
- **generate_if_with_label** - Similar reasoning, useful companion rule.
- **prefix_instance** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 27.4 Loop generate constructs
- 27.6 External names for unnamed generate blocks
