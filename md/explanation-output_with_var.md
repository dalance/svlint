This check mandates that each `output` port must be explicitly declared as a
variable, rather than the default nettype or implict datakind.

The rules for determining port kind, datatype, and direction are specified in
IEEE1800-2017 Clause 23.2.2.3 and facilitate various shorthand notations which
are backwards compatible with the semantics of Verilog (IEEE1364-1995):
- `output a` -> `output tri logic a` The implicit datatype is `logic` and the
  default nettype is `tri` (without overriding via the `` `default_nettype ``
  compiler directive).
- `output wire a` -> `output tri logic a` Again, using the implicit datatype of
  `logic`;
  As `wire` is an alias for `tri`, this is equivalent to the above example.
- `output wire logic a` -> `output tri logic a` Again, even with an explicit
  datatype (`logic`), the `wire` keyword is simply an alias for the datakind
  `tri`.
- `output logic a` -> `output var logic a` This time the datakind is implicit,
  but the datatype is *explicit*, so the inferred datakind is `var`.

When the datatype is implicit and the default nettype is overridden to none,
i.e. with the compiler directive `` `default_nettype none ``,  output ports
require an explicit datakind.

Although the semantics of `output a` are equivalent in IEEE1364-1995, the
intent is not clearly described, and the difference to `output logic a` is
unintuitive.
An author should use `output` to declare ports which should only be driven
internally, and `inout` to declare ports which may also be driven externally.
In order to describe the intended uni-directional behavior, `output` ports must
be declared with an explicit `var` datakind, thus requiring the compiler to
check that the output is only driven from within the module (otherwise, emit an
error).

See also:
  - **default_nettype_none** - Useful companion rule.
  - **inout_with_tri** - Suggested companion rule.
  - **output_with_var** - Suggested companion rule.
  - **prefix_output** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.5 Nets and variables
  - 6.6 Net types
  - 22.8 default nettype
  - 23.2.2 Port declarations
