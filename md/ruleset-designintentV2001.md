
This ruleset has the same aims as **ruleset-designintent** but with the
additional aim of only allowing code which is backwards compatible with
IEEE1364-2001 (Verilog).
Note that IEEE1364-2001 is not the most recent version (IEEE1364-2005), which
was released in the same year as the first version of SystemVerilog
(IEEE1800-2005).

Firstly, let's forbid some things which are only in SystemVerilog, but not
Verilog.
```toml
syntaxrules.keyword_forbidden_always_comb = true
syntaxrules.keyword_forbidden_always_ff = true
syntaxrules.keyword_forbidden_always_latch = true
syntaxrules.keyword_forbidden_priority = true
syntaxrules.keyword_forbidden_unique = true
syntaxrules.keyword_forbidden_unique0 = true
#syntaxrules.keyword_forbidden_logic = true # TODO
syntaxrules.operator_incdec = true
syntaxrules.operator_self_assignment = true
```

Next, let's use some of the rules in common with **ruleset-simsynth**.
```toml
syntaxrules.enum_with_type = true
syntaxrules.function_with_automatic = true
syntaxrules.operator_case_equality = true
syntaxrules.action_block_with_side_effect = true
syntaxrules.default_nettype_none = true
syntaxrules.function_same_as_system_function = true
```

Verilog does allow both ANSI and non-ANSI forms of module declaration, but
there is a crucial difference for the ANSI form:
Only `parameter`s are allowed in the list of parameter ports, not
`localparam`s, meaning that derived parameters are overridable.
In the following example, there is no way of preventing `PTR_W` from being
overridden to something incorrect, risking some frustration and wasted time
when non-obvious effects cause issues later.
```verilog
module M
  #(parameter integer WIDTH = 123
  , parameter integer PTR_W = clogb2(WIDTH)
  )
  ( input  wire [WIDTH-1:0] i_data
  , output wire [PTR_W-1:0] o_pointer
  );
```
However, using the non-ANSI form allows `PTR_W` to be specified as
`localparam`, thus preventing overrides and the resulting confusion, i.e:
```verilog
module M
  ( i_data
  , o_pointer
  );

  parameter integer WIDTH = 123;
  localparam integer PTR_W = clogb2(WIDTH);

  input  wire [WIDTH-1:0] i_data;
  output wire [PTR_W-1:0] o_pointer;
```
While this only affects modules which use derived parameters in the port
declarations, a consistent style is generally easier to work with.
For these reasons, the non-ANSI form is required.
```toml
syntaxrules.module_ansi_forbidden = true
```

SystemVerilog introduced several keywords which greatly help to clarify intent,
but these are unavailable.
Instead of `always_ff @(posedge clk)` and `always_comb`, we can use
`always @(posedge clk)` and `always @*`.
That means only the form like `always @(a or b)`, i.e. no edge sensitivities,
can be forbidden.
```toml
syntaxrules.general_always_level_sensitive = true
```
On the same theme, guidelines around blocking vs non-blocking assignments also
need to be altered, but keeping the same general intention.
Clocked `always` processes should only use non-blocking assignment `<=`, and
combinatorial `always` processes should only use blocking assignment `=`.
```toml
syntaxrules.blocking_assignment_in_always_at_edge = true
syntaxrules.non_blocking_assignment_in_always_no_edge = true
```

Verilog doesn't have the same distinction between 2-state and 4-state types as
SystemVerilog, e.g. `int` and `integer`, but requiring some type is still a
good idea.
```toml
syntaxrules.localparam_explicit_type = true
syntaxrules.parameter_explicit_type = true
syntaxrules.parameter_default_value = true
syntaxrules.parameter_in_generate = true
```

In IEEE1364-2001, the use of `generate` and `endgenerate` is mandatory, but
optional in IEEE1364-2005.
For more compatibility, these keywords are required by this ruleset, as are
`genvar` declarations outside their generate `for` statements.
The enablements of these rules are swapped in **ruleset-designintent** to
reduce visual noise in SystemVerilog.
```toml
syntaxrules.genvar_declaration_in_loop = false
syntaxrules.genvar_declaration_out_loop = true
syntaxrules.keyword_forbidden_generate = false
syntaxrules.keyword_required_generate = true
```

Unlike the in the richer language of SystemVerilog, forbidding sequential
blocks (between `begin` and `end`) and sequential loops (`for` under `always`)
is probably too restrictive for Verilog.
Indeed, there is little point in using `always @*` instead of `assign` if
`begin` and `end` are forbidden - in SystemVerilog, `always_comb` provides
extra compile-time checks that `assign` does not.
```toml
#syntaxrules.loop_statement_in_always = true # Not implemented.
#syntaxrules.sequential_block_in_always = true # Not implemented.
syntaxrules.case_default = true # Applies in functions.
syntaxrules.explicit_case_default = true # Applies under `always`.
syntaxrules.explicit_if_else = true
syntaxrules.multiline_for_begin = true
syntaxrules.multiline_if_begin = true
```
