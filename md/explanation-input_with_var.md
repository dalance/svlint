This check mandates that each `input` port must be explicitly declared as a
variable, rather than the default nettype.

The rules for determining port kind, datatype, and direction are specified in
IEEE1800-2017 Clause 23.2.2.3 and facilitate various shorthand notations which
are backwards compatible with the semantics of Verilog (IEEE1364-1995):
- `input a` -> `input tri logic a` The implicit datatype is `logic` and the
  default nettype is `tri` (without overriding via the `` `default_nettype ``
  compiler directive).
- `input wire a` -> `input tri logic a` Again, using the implicit datatype of
  `logic`;
  As `wire` is an alias for `tri`, this is equivalent to the above example.
- `input logic a` -> `input tri logic a` This time using an explicit datatype
  (`logic`) but relying on the default nettype for its datakind.
- `input wire logic a` -> `input tri logic a` Again, even with an explicit
  datatype (`logic`), the `wire` keyword is simply an alias for the datakind
  `tri`.

When the default nettype is overridden to none, i.e. with the compiler
directive `` `default_nettype none ``, input ports require an explicit
datakind.

Although the semantics of `input a` are equivalent in IEEE1364-1995, the intent
is not clearly described.
An author should use `input` to declare ports which should only be driven
externally, and `inout` to declare ports which may also be driven internally.
In order to describe the intended uni-directional behavior, `input` ports must
be declared with an explicit `var` datakind, thus requiring the compiler to
check that the input is not driven from within the module (and if so, emit an
error).

See also:
  - **default_nettype_none** - Useful companion rule.
  - **inout_with_tri** - Suggested companion rule.
  - **output_with_var** - Suggested companion rule.
  - **prefix_input** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.5 Nets and variables
  - 6.6 Net types
  - 22.8 default nettype
  - 23.2.2 Port declarations
