It is legal for parameters declared in a parameter port list to omit
a default value (an elaboration-time constant), thus setting to parameter value
to the default value of its type when not overridden.
This language feature can be used by module authors to force integrators to
choose an override value, by ensuring that the default is invalid.

```systemverilog
module M #(parameter int NE0) ();
  if (0 == NE0) $error("MUST_OVERRIDE must not be zero.");
endmodule

module Parent ();
  M u_bad (); // This causes elaboration error.
  M #(NE0=1) u_good ();
endmodule
```

The example above uses a system elaboration task to explicitly force an
elaboration error, but there are several ways to implictly cause elaboration
errors.
Relying on the type's default value can cause problems for two reasons:
1. Some tools do not support this syntax.
2. Simply omitting the default value is unclear about the author's intention,
  particularly when the type is also omitted.

This rule checks that all parameter ports have an explicit default value.

See also:
  - **parameter_explicit_type** - Useful companion rule.
  - **parameter_type_twostate** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.20.1 Parameter declaration syntax
  - 6.20.2 Value parameters
  - 23.2.3 Parameterized modules
  - A.10 Footnotes (normative), number 18.
