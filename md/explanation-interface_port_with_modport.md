A SystemVerilog Interface (SVI) defines a set of named signals which can be
used in many places within a design.
For example, if modules `A` and `B` both instance an interface `I` as `A.u_I`
and `B.u_I`, then both modules get their own collection of named signals,
accessed like `u_I.x`.
Each interface instance is separate, so `A.u_I.x` is independent of `B.u_I.x`.
By adding another signal `y` to the interface, two new signals are created,
`A.u_I.y` and `B.u_I.y`.

SVIs are useful for connecting hierarchical modules with a minimal amount of
code, i.e. by using interface ports.
To specify the direction of signals in an SVI, a `modport` is declared with
and identifier and the directions of each signal declared from the perspective
of inside a module.
Without a `modport`, the default direction of interface port signals is
`inout`.
This is often undesirable for synthesizable digital designs, so this rule
requires that each interface port includes a modport identifier.

See also:
  - **inout_with_tri** - Useful companion rule.
  - **input_with_var** - Useful companion rule.
  - **non_ansi_module** - Useful companion rule.
  - **output_with_var** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 25.4 Ports in interfaces
  - 25.5 Modports
