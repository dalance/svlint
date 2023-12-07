
Rules that forbid suspicious constructions, i.e. ways of specifying hardware
that are legal according to the LRM, but may express their intention unclearly.

This ruleset is a superset of **ruleset-simsynth**.
These rules don't depend on each other or interact to provide additional
properties.
```toml
syntaxrules.blocking_assignment_in_always_ff = true
syntaxrules.blocking_assignment_in_always_latch = true
syntaxrules.non_blocking_assignment_in_always_comb = true
syntaxrules.case_default = true
syntaxrules.enum_with_type = true
syntaxrules.function_with_automatic = true
syntaxrules.keyword_forbidden_priority = true
syntaxrules.keyword_forbidden_unique = true
syntaxrules.keyword_forbidden_unique0 = true
#syntaxrules.general_always_no_edge = true # Redundant with keyword_forbidden_always.
syntaxrules.operator_case_equality = true
```

This ruleset has further rules which don't depend on each other or combine
to provide additional properties.
Please see their individual explanations for details.
Note, in the related **ruleset-verifintent**, the rule
**keyword_forbidden_always** is not enabled because it is perfectly reasonable
for a simulation testbench to schedule assignments, tasks, and functions in
ways that wouldn't make sense for synthesizable hardware.
```toml
syntaxrules.action_block_with_side_effect = true
syntaxrules.default_nettype_none = true
syntaxrules.function_same_as_system_function = true
syntaxrules.keyword_forbidden_always = true
syntaxrules.keyword_forbidden_wire_reg = true
syntaxrules.module_nonansi_forbidden = true
syntaxrules.procedural_continuous_assignment = true
```

When synthesised into a netlist, generate blocks should have labels so that
their inferred logic can be detected in hierarchical paths.
Although the LRM is clear about the implict naming of unlabelled generate
blocks (see IEEE1800-2017 clause 27.6), using a well-named label provides some
clarification about the intention behind that logic.
In the similar **ruleset-verifintent**, these rules are not enabled because
they (mostly) relate to synthesizable hardware.
```toml
syntaxrules.generate_case_with_label = true
syntaxrules.generate_for_with_label = true
syntaxrules.generate_if_with_label = true
```

Generally, elaboration-time constants (`parameter`, `localparam`) should be
2-state types and declared with a default value.
Additionally, where the context defines that `parameter` is an alias for
`localparam`, authors should demonstate that they understand the constant
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

Rules in the below subset combine to provide an important property for the
robust design of synthesizable hardware - that you can easily draw a schematic
of what the synthesis result should look like.
The two rules of thumb are to always fully specify decision logic, and never
use procedural models for (what will be synthesized to) parallel logic.
```toml
syntaxrules.explicit_case_default = true
syntaxrules.explicit_if_else = true
syntaxrules.loop_statement_in_always_comb = true
syntaxrules.loop_statement_in_always_ff = true
syntaxrules.loop_statement_in_always_latch = true
syntaxrules.sequential_block_in_always_comb = true
syntaxrules.sequential_block_in_always_ff = true
syntaxrules.sequential_block_in_always_latch = true
```

Where sequential modelling of parallel logic is an unavoidable pragmatic
approach, using the `begin` and `end` keywords should be done carefully with
proper indentation.
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

