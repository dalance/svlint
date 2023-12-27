
Rules that forbid suspicious constructions, i.e. those which are legal
according to the LRM, but may express their intention unclearly.
This ruleset is similar to **ruleset-designintent**, but with some rules
enabled or disabled where they are applicable to testbench code (instead of
synthesizable digital logic).

While this ruleset is *not* a superset of **ruleset-simsynth**, some of those
rules are also useful for testbench/verification code.
A clean separation of (non-)blocking assignments and `always_(comb|ff)`
processes is useful to prevent the specification of processes with scheduling
semantics which are difficult to reason about.

```toml
syntaxrules.blocking_assignment_in_always_ff = true
syntaxrules.non_blocking_assignment_in_always_comb = true
syntaxrules.enum_with_type = true
syntaxrules.keyword_forbidden_priority = true
syntaxrules.keyword_forbidden_unique = true
syntaxrules.keyword_forbidden_unique0 = true
syntaxrules.procedural_continuous_assignment = true
```

This ruleset has further rules which don't depend on each other or combine
to provide additional properties.
Please see their individual explanations for details.
Note, in the related **ruleset-designintent**, an additional rule
**keyword_forbidden_always** is enabled.
```toml
syntaxrules.action_block_with_side_effect = true
syntaxrules.default_nettype_none = true
syntaxrules.function_same_as_system_function = true
syntaxrules.keyword_forbidden_wire_reg = true
syntaxrules.module_nonansi_forbidden = true
```

Generally, elaboration-time constant (`parameter`, `localparam`) should be
2-state types and always supplied with some default value.
Additionally, where the context defines that `parameter` is an alias for
`localparam`, author's should demonstate that they understand the constant
cannot be overriden by using the `localparam` keyword.
```toml
syntaxrules.localparam_type_twostate = true
syntaxrules.parameter_type_twostate = true
syntaxrules.localparam_explicit_type = true
syntaxrules.parameter_explicit_type = true
syntaxrules.parameter_default_value = true
syntaxrules.parameter_in_generate = true
syntaxrules.parameter_in_package = true
```

Genvars, which are also elaboration-time constants, should be declared within
generate `for` loops to reduce their scope.
This allows readers to be confident that they can see all of the relevant
information about a genvar in one place, i.e. declaration and usage.
A notable advantage of declaring genvars in each generate loop is that authors
are encouraged to give their genvars suitably descriptive names.
Rules on the use of the `generate` and `endgenerate` keywords is similarly
subjective, but this ruleset forbids their use because readers should be aware
that all `case`, `for`, and `if` blocks outside of assignment processes are
generate blocks.
Further, the use of `generate` and `endgenerate` is entirely optional with no
semantic difference to not using them.
```toml
syntaxrules.genvar_declaration_in_loop = true
syntaxrules.genvar_declaration_out_loop = false
syntaxrules.keyword_forbidden_generate = true
syntaxrules.keyword_required_generate = false
```

To prevent difficult-to-read procedural code, using the `begin` and `end`
keywords should be done carefully with proper indentation.
Note, this ruleset does *not* check the amount of indentation like
**style_indent**.
```toml
syntaxrules.multiline_for_begin = true
syntaxrules.multiline_if_begin = true
```

The semantics around port declarations are, perhaps, unintuitive but were
designed for backward compliance with Verilog (IEEE1364-1995).
The below subset ensures that port declarations clearly convey important
information about the direction and update mechanism of each signal port.
```toml
syntaxrules.inout_with_tri = true
syntaxrules.input_with_var = true
syntaxrules.output_with_var = true
syntaxrules.interface_port_with_modport = true
```

