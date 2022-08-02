This document is generated from the rules' source code (`svlint/src/rules/*.rs`)
and testcases (`testcases/(fail|pass)/*.sv`) using the `mdgen` utility.

# Rules

Each rule is documented with 5 pieces of information:
- Hint: A brief instruction on how to modify failing SystemVerilog.
  Also displayed in supported editors using [svls](https://github.com/dalance/svls).
- Reason: A one sentence explanation of the rule's purpose.
  Also displayed in supported editors using [svls](https://github.com/dalance/svls).
- Pass Example: A valid piece of SystemVerilog which is known to pass the rule.
  Ideally, this will show an example of best-practice.
- Fail Example: A valid piece of SystemVerilog which is known to fail the rule.
  In some cases the code shows multiple commented examples.
- Explanation: A full explanation of the rule's purpose with references to any
  other relevant information sources.

---
## `blocking_assignment_in_always_ff`

### Hint

Do not use blocking assignments within `always_ff`.

### Reason

Blocking assignment in `always_ff` may cause undefined event ordering.

### Pass Example

```SystemVerilog
module M;
always_ff @(posedge clk) q1 <= d; // Correct.

/* svlint off blocking_assignment_in_always_ff */
always_ff @(posedge clk) q2 = d;  // Control comments avoid failure.
/* svlint on blocking_assignment_in_always_ff */
endmodule
```

### Fail Example

```SystemVerilog
module M;
/* svlint off blocking_assignment_in_always_ff */
always_ff @(posedge clk) q1 = d;   // Control comments avoid failure.
/* svlint on blocking_assignment_in_always_ff */

always_ff @(posedge clk) q2 = d;   // Failure.
endmodule
```

### Explanation

Simulator event ordering between blocking and non-blocking assignments
is undefined, so observed behavior simulator-dependent.
As all examples in IEEE1800-2017 show, `always_ff` should only contain
non-blocking assignments in order for sampling and variable evaluation
to operate in a defined order.

Specifically, `always_ff` constructs should not contain blocking assignments:
  - Blocking assignment operator, e.g. `foo = 123;`
  - Increment/decrement operators, e.g. `foo++;`, `foo--;`.

The most relevant clauses of IEEE1800-2017 are:
  - 9.2.2.4 Sequential logic always_ff procedure
  - 9.4.2 Event control
  - 10.4.1 Blocking procedural assignments
  - 10.4.2 Nonblocking procedural assignments
  - 16.5.1 Sampling


---
## `case_default`

### Hint

Use a `default` expression in `case` statements.

### Reason

Incomplete case may cause simulation/synthesis mismatch in `always_comb` and `function`.

### Pass Example

```SystemVerilog
module A;
always_comb begin
    case (x)
        1: y = 0;
        default: y = 0;
    endcase
end
always_ff begin
    case (x)
        1: y = 0;
    endcase
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always_comb begin
    case (x)
        1: y = 0;
    endcase
end
always_ff begin
    case (x)
        1: y = 0;
    endcase
end
endmodule
```

### Explanation

IEEE1800-2017 (clause 9.2.2.2) comments that tools should *warn* if an
`always_comb` procedure infers memory.
However, simulators and synthesis tools are not required to enforce that
`always_comb` procedures only infer combinational logic.
This allows for simulators and synthesis tools to interpret these procedures
differently, which results in a mismatch between simulation and synthesis.

An incomplete case statement may be interpreted as latched logic,
e.g: `always_comb case (foo) '0: a = 5; endcase`.
Only the case where `foo == 0` is specified, to update variable `a` to the
value `5`.
When `foo` is non-zero, this example may be interpreted in at least two ways:
  - `a = 'x;` - As the new value is not specified, it is unknown.
    A synthesis tool may allow node `a` to be undriven, or choose to drive
    `a` equivalently to one of the explicitly specified case expressions.
  - `a = a;` - As the new value is not specified, do not change `a`.
    A synthesis tool may produce a latching circuit.

The most relevant clauses of IEEE1800-2017 are:
  - 9.2.2.2 Combinational logic `always_comb` procedure
  - 12.5 Case statement
  - 13.4 Functions


---
## `default_nettype_none`

### Hint

Place `` `default_nettype none`` at the top of source code.

### Reason

Compiler directive `` `default_nettype none`` detects unintentional implicit wires.

### Pass Example

```SystemVerilog
`default_nettype none
module A;
endmodule

```

### Fail Example

```SystemVerilog
module A;
endmodule

```

### Explanation

The `` `default_netype`` compiler directive can be used to specify the net type
of implicit nets, i.e. where a signal is referenced, or assigned to, without
being declared.
IEEE1800-2017 clause 22.8 stipulates "When no `` `default_nettype`` directive
is present or if the `` `resetall`` directive is specified, implicit nets are of
type `wire`."

SystemVerilog makes a distiction between variables (only 0 or 1 drivers)
and nets (0 or more drivers).
IEEE1364-2001 (Verilog) uses variables as abstractions for data storage
elements (`reg`, `integer`, `real`, `time`, `realtime`).
In contrast, IEEE1800-2017 (SystemVerilog) the distinction between nets and
variables is defined by how a simulator must calculate a value.
In a simulator, a variable stores a value, but a net's value is calculated by
evaluating the strength of all drivers.
To keep compatibility with Verilog, the default net type of an undeclared net
in SystemVerilog is `wire` (a net, not a variable), which requires evaluating a
list of values with strengths, rather than simply looking up a value.
The distiction between data storage elements and physical wires is therefore
made in using `always_comb`, `always_ff`, and (less commonly) `always_latch`
keywords.

Variables are preferred over nets for most digital logic for 2 reasons:
  - Only 0 or 1 drivers allowed, so an accidental multi-driving is caught by
    a compile time error.
  - Simulator performance (dependent on implemetation).
    Value can be found by lookup, rather than evaluation of drivers.
When `` `default_nettype none`` is used, all signals must be declared, thus
forcing the author to consider whether they mean a variable or a net.

The most relevant clauses of IEEE1800-2017 are:
  - 6.5 Nets and variables
  - 22.8 default nettype

Note: One prominent paper (Cliff Cummings, HDLCON 2002) recommends *against*
using `` `default_nettype none`` on the basis that concise, typeless code has
fewer opportunities for mistakes.
This attitude was popular at the time, e.g. Python's dynamic typing, but
modern attitudes are now favouring explicit types, e.g. Python's new type
checking syntax and tooling.
Additionally, the reasoning behind this guideline only applies principally to
IEEE1364, but not strongly to IEEE1800.


---
## `enum_with_type`

### Hint

Specify an explicit `enum` base type.

### Reason

The default `enum` base type is `int` (32b, 2-state).

### Pass Example

```SystemVerilog
module A;
typedef enum logic {
    C
} B;
endmodule
```

### Fail Example

```SystemVerilog
module A;
typedef enum {
    C
} B;
endmodule
```

### Explanation

SystemVerilog has both 2-state types (each bit can take the values 0 or 1),
and 4-state types (each bit can take the values 0, 1, x, or z).
2-state types are useful for holding constants, and programming
non-synthesizable simulation constructs.
4-state types are useful for modelling physical hardware because undriven,
multiply-driven, or improperly-driven wires can hold unknown states that
cannot be sufficiently modelled by only 2 states.
Therefore, it is important to use the 4-state types when writing SystemVerilog
which will be used to infer physical hardware.

For example, a counter described as
`always_ff @(posedge clk) count_q <= count_q + 'd1;`
should be declared like `logic [4:0] count_q;`.
This infers 5 non-reset flip-flops where the initial value is unknown, and in a
4-state simulation the value of `count_q` is always unknown (`'x`, because
there's no initialization).
Instead, if it was declared as `bit [4:0] count_q;`, then the initial value
is `5'd0`, so a simulation will show `count_q` changing on every positive
edge of `clk`.
When describing physical hardware, it would be useful to know that the inferred
flip-flops have no reset, i.e., you want to be *able* to see x's when a mistake
is made even if you don't want to see x's.

An `enum` is a set of named values of a single type.
If no data type is specified, then the default `int` (32b, 2-state) is implied.
For example, `enum {RED, BLACK} m; assign m = foo ? BLACK : RED;`
describes a multiplexor, but a simulator is unable to sufficiently model the
behavior of `m` when the value of `foo` is unknown.
A more appropriate declaration is
`typedef enum int {RED, BLACK} color; integer m;`.

Note: Comparison of 4-state variables against 2-state constants/enums *is*
appropriate, e.g. `logic a; a = (m == RED);`.

The most relevant clauses of IEEE1800-2017 are:
  - 6.8 Variable declarations
  - 6.11 Integer data types
  - 6.19 Enumerations
  - Table 6.7 Default variable initial values
  - Table 6.8 Integer data types


---
## `explicit_case_default`

### Hint

Add a `default` arm to the `case` statement.

### Reason

Fully-specified case clarifies design intent.

### Pass Example

```SystemVerilog
module A;
always_comb begin
    case (x)
        1: y = 0;
        default: y = 0;
    endcase
end
always_ff begin
    case (x)
        1: y = 0;
        default: y = 0;
    endcase
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always_comb begin
    case (x)
        1: y = 0;
    endcase
end
always_ff begin
    case (x)
        1: y = 0;
    endcase
end
endmodule
```

### Explanation

The reasoning behind this rule are different between combinatial constructs
(`always_comb`, `always @*`) vs sequential constructs (`always_ff`,
`always_latch`).
The reasoning behind this rule is equivalent to that of **explicit_if_else**.

For combinational constructs, the reasoning behind this rule is equivalent to
that of the rule **case_default**.
To summarize, an incompletely-specified case statement may infer sequential
behavior (i.e. memory), thus causing a mismatch between simulation and synthesis
tools.
Due to the slightly different formulations, it is recommended that both this
rule and **case_default** are enabled.

For sequential constructs, the reasoning behind this rule is equivalent to
those of the rules **sequential_block_in_always_ff** and
**sequential_block_in_always_latch**.
To summarize, fully-specified case statements make the design intent explicit
and clear through some useful redundancy.

NOTE: The legacy keyword `always` can infer both combinational and sequential
constructs in the same block, which can be confusing and should be avoided.
Use of the legacy keyword can be detected with the rule **legacy_always**.

See also:
  - **case_default** - Useful companion rule.
  - **explicit_if_else** - Useful companion rule.
  - **legacy_always** - Useful companion rule.
  - **sequential_block_in_always_comb** - Useful companion rule.
  - **sequential_block_in_always_ff** - Useful companion rule.
  - **sequential_block_in_always_latch** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.5 Case statement


---
## `explicit_if_else`

### Hint

Add an `else` clause to the `if` statement.

### Reason

Fully-specified conditional clarifies design intent.

### Pass Example

```SystemVerilog
module A;
always_ff
  if (x) y <= 0;
  else   y <= z;
always_comb
  if (x) y = 0;
  else   y = z;
endmodule
```

### Fail Example

```SystemVerilog
module A;
always_ff if (x) y <= 0;
always_comb if (x) y = 0;
endmodule
```

### Explanation

The reasoning behind this rule are different between combinatial constructs
(`always_comb`, `always @*`) vs sequential constructs (`always_ff`,
`always_latch`).
The reasoning behind this rule is equivalent to that of **explicit_case_default**.

For combinational constructs, the reasoning behind this rule is equivalent to
that of the rule **case_default**.
To summarize, an incompletely-specified case statement may infer sequential
behavior (i.e. memory), thus causing a mismatch between simulation and synthesis
tools.

For sequential constructs, the reasoning behind this rule is equivalent to
those of the rules **sequential_block_in_always_ff** and
**sequential_block_in_always_latch**.
To summarize, fully-specified case statements make the design intent explicit
and clear through some useful redundancy.

NOTE: The legacy keyword `always` can infer both combinational and sequential
constructs in the same block, which can be confusing and should be avoided.
Use of the legacy keyword can be detected with the rule **legacy_always**.

See also:
  - **explicit_case_default** - Useful companion rule.
  - **legacy_always** - Useful companion rule.
  - **sequential_block_in_always_comb** - Useful companion rule.
  - **sequential_block_in_always_ff** - Useful companion rule.
  - **sequential_block_in_always_latch** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.4 Conditional if-else statement


---
## `function_same_as_system_function`

### Hint

Rename `function` to something other than the name of a built-in function.

### Reason

Name clashes may cause confusion amongst tools and readers.

### Pass Example

```SystemVerilog
module A;
function my_clog2;
endfunction
endmodule
```

### Fail Example

```SystemVerilog
module A;
function clog2;
endfunction
endmodule
```

### Explanation

IEEE1800-2017 provides a variety of built-in functions, which must be
implemented in simulation and synthesis tools.
This rule is designed to catch (possibly incorrect) re-implementations of these
functions which may have different behavior and confuse readers.
Additionally, some tools may (wrongly) confuse user-defined functions with the
built-in system of the same name (except of the leading `$`) which may lead
to inconsistent results between tools.

The most relevant clauses of IEEE1800-2017 are:
  - 13.7 Task and function names
  - 20 Utility system tasks and system functions
  - 23.8.1 Task and function name resolution


---
## `function_with_automatic`

### Hint

Add the `automatic` lifetime specifier to function.

### Reason

Static lifetime of function items causes a simulation/synthesis mismatch.

### Pass Example

```SystemVerilog
module A;
function automatic A;
endfunction
endmodule

// default lifetime
module automatic A;
function A;
endfunction
endmodule

interface automatic A;
function A;
endfunction
endinterface

program automatic A;
function A;
endfunction
endprogram

package automatic A;
function A;
endfunction
endpackage

// override default lifetime
module static A;
function automatic A;
endfunction
endmodule

interface static A;
function automatic A;
endfunction
endinterface

program static A;
function automatic A;
endfunction
endprogram

package static A;
function automatic A;
endfunction
endpackage

// function in class is automatic
module A;
class A;
function A;
endfunction
endclass
endmodule

module automatic A;
class A;
function A;
endfunction
endclass
endmodule

module static A;
class A;
function A;
endfunction
endclass
endmodule
```

### Fail Example

```SystemVerilog
module A;
function A;
endfunction
endmodule
```

### Explanation

Functions defined within a module, interface, program, or package default to
being static, with all declared items being statically allocated.
These items shall be shared across all uses of the function executing
concurrently.
This causes a mismatch between simulation and synthesis.

Functions can be defined to use automatic storage by using the `automatic`
keyword as part of the function declaration, i.e. in simulation each use of a
function is allocated dynamically for each concurrent function call.
This behavior can be accurately inferred in synthesis.

The most relevant clauses of IEEE1800-2017 are:
  - 13.4.2 Static and automatic functions


---
## `generate_case_with_label`

### Hint

Use a label with prefix "l_" on conditional generate block.

### Reason

Unnamed generate blocks imply unintuitive hierarchical paths.

### Pass Example

```SystemVerilog
module A;
generate case (2'd3)
  2'd1:     begin: l_nondefault wire c = 1'b0; end
  default:  begin: l_default    wire c = 1'b0; end
endcase endgenerate
endmodule
```

### Fail Example

```SystemVerilog
module A;
generate case (2'd0)
  2'd1:     wire a = 1'b0; // nondefaultNoBegin
  default:  wire a = 1'b0; // defaultNoBegin
endcase endgenerate
generate case (2'd1)
  2'd1:     begin wire b = 1'b0; end // nondefaultNoLabel
  default:  begin wire b = 1'b0; end // defaultNoLabel
endcase endgenerate
generate case (2'd2)
  2'd1:     begin: nondefaultNoPrefix wire c = 1'b0; end
  default:  begin: noPrefix           wire c = 1'b0; end
endcase endgenerate
endmodule
```

### Explanation

Conditional generate constructs select zero or one blocks from a set of
alternative generate blocks within a module, interface, program, or checker.
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
  - **generate_for_with_label** - Similar reasoning, useful companion rule.
  - **generate_if_with_label** - Equivalent reasoning, useful companion rule.
  - **prefix_instance** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 27.5 Conditional generate constructs
  - 27.6 External names for unnamed generate blocks


---
## `generate_for_with_label`

### Hint

Use a label with prefix "l_" on loop generate block.

### Reason

Unnamed generate blocks imply unintuitive hierarchical paths.

### Pass Example

```SystemVerilog
module A;
for(genvar i=0; i<10; i++) begin: l_a
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
for(genvar i=0; i<10; i++) foo[i] = i;// noBegin
for(genvar i=0; i<10; i++) begin // noLabel
end
endmodule
```

### Explanation

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


---
## `generate_if_with_label`

### Hint

Use a label with prefix "l_" on conditional generate block.

### Reason

Unnamed generate blocks imply unintuitive hierarchical paths.

### Pass Example

```SystemVerilog
module A;
if (a) begin: l_abc
end else if (b) begin: l_def
end else begin: l_hij
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
if (a) begin
end else if (b) begin
end else begin
end

if (c) begin: abc
end else if (d) begin: def
end else begin: hij
end

if (e) begin: l_klm
end else begin: mno
end
endmodule
```

### Explanation

Conditional generate constructs select zero or one blocks from a set of
alternative generate blocks within a module, interface, program, or checker.
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
  - **generate_case_with_label** - Equivalent reasoning, useful companion rule.
  - **generate_for_with_label** - Similar reasoning, useful companion rule.
  - **prefix_instance** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 27.5 Conditional generate constructs
  - 27.6 External names for unnamed generate blocks


---
## `generate_keyword_forbidden`

### Hint

Remove `generate`/`endgenerate` keywords.

### Reason

Keywords `generate`/`endgenerate` do not change semantics of generate blocks.

### Pass Example

```SystemVerilog
module A;
endmodule
```

### Fail Example

```SystemVerilog
module A;
generate
endgenerate
endmodule
```

### Explanation

The `generate`/`endgenerate` keywords may be used in a module, interface,
program, or checker to define a generate region.
A generate region is a textual span in the module description where generate
constructs may appear.
Use of generate regions is optional.
There is no semantic difference in the module when a generate region is used.
A parser may choose to recognize the generate region to produce different error
messages for misused generate construct keywords.

As the semantics of generate blocks are unchanged by the
`generate`/`endgenerate` keywords, the keywords can be argued to be visual
noise, simply distracting the reader.
Therefore, this rule is designed to detect and forbid their use.

NOTE: Some non-compliant tools may require the use of these keywords, which
provides an argument against this rule.

See also:
  - **generate_keyword_required** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.3 Generate construct syntax


---
## `generate_keyword_required`

### Hint

Use `generate`/`endgenerate` keywords to define generate regions.

### Reason

Omitting `generate`/`endgenerate` keywords may cause issues with non-compliant tools.

### Pass Example

```SystemVerilog
module A;
generate
if (a) begin
end
case (a)
    default: a;
endcase
for(i=0; i<10; i++) begin
end
endgenerate
endmodule
```

### Fail Example

```SystemVerilog
module A;
if (a) begin
end
case (a)
    default: a;
endcase
for(i=0; i<10; i++) begin
end
endmodule
```

### Explanation

The `generate`/`endgenerate` keywords may be used in a module, interface,
program, or checker to define a generate region.
A generate region is a textual span in the module description where generate
constructs may appear.
Use of generate regions is optional.
There is no semantic difference in the module when a generate region is used.
A parser may choose to recognize the generate region to produce different error
messages for misused generate construct keywords.

Some non-compliant tools may require the use of these keywords.
Therefore, this rule is designed to mandate their use.

NOTE: The visual noise introduced by these keywords provides an argument
against this rule.

See also:
  - **generate_keyword_forbidden** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.3 Generate construct syntax


---
## `genvar_declaration_in_loop`

### Hint

`genvar` must be declared in loop

### Reason

the scope of variable should be minimized

### Pass Example

```SystemVerilog
module A;
for(genvar i=0;i<10;i++) begin: a
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
genvar i;
for(i=0;i<10;i++) begin
end
endmodule
```

### Explanation

TODO


---
## `genvar_declaration_out_loop`

### Hint

`genvar` must be declared out loop

### Reason

some tools don't support `genvar` declaration in loop

### Pass Example

```SystemVerilog
module A;
genvar i;
for(i=0;i<10;i++) begin: a
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
for(genvar i=0;i<10;i++) begin
end
endmodule
```

### Explanation

TODO


---
## `inout_with_tri`

### Hint

`inout` must have `tri`

### Reason



### Pass Example

```SystemVerilog
module A (
    inout tri a
);
endmodule
```

### Fail Example

```SystemVerilog
module A (
    inout wire a
);
endmodule
```

### Explanation

TODO


---
## `input_with_var`

### Hint

`input` must have `var`

### Reason

`input wire` can be assigned by mistake. `input logic` becomes error with `default nettype none` because it doesn't have net type.

### Pass Example

```SystemVerilog
module A (
    input var a
);
endmodule
```

### Fail Example

```SystemVerilog
module A (
    input logic a
);
endmodule
```

### Explanation

TODO


---
## `interface_port_with_modport`

### Hint

interface port must have modport

### Reason

interface port without modport maybe `inout` at synthesis

### Pass Example

```SystemVerilog
module A (
    test_if.a a,
    interface.b b
);
endmodule
```

### Fail Example

```SystemVerilog
module A (
    test_if a,
    interface b
);
endmodule
```

### Explanation

TODO


---
## `legacy_always`

### Hint

`always_comb`/`always_ff`/`always_latch` must be used

### Reason

`always` can't detect blocking/non-blocking mistake

### Pass Example

```SystemVerilog
module A;
always_comb begin
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always @* begin
end
endmodule
```

### Explanation

TODO


---
## `level_sensitive_always`

### Hint

level sensitive `always` must be `always_comb`

### Reason

`always` can't detect blocking/non-blocking mistake

### Pass Example

```SystemVerilog
module A;
always_comb begin
end
always_comb begin
end
always @ ( posedge a ) begin
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always @* begin
end
always @ ( a or b ) begin
end
always @ ( posedge a ) begin
end
endmodule
```

### Explanation

TODO


---
## `localparam_explicit_type`

### Hint

`localparam` must be have an explicit type

### Reason

parameter types show intent and improve readability

### Pass Example

```SystemVerilog
module A;
localparam int a = 0;
endmodule
```

### Fail Example

```SystemVerilog
module A;
localparam a = 0;
endmodule
```

### Explanation

TODO


---
## `localparam_type_twostate`

### Hint

Declare `localparam` with an explicit 2-state type.

### Reason

Design constants with Xs or Zs may cause simulation/synthesis mismatch.

### Pass Example

```SystemVerilog
module A;
  localparam byte     a = 0; // 8b
  localparam shortint b = 0; // 16b
  localparam int      c = 0; // 32b
  localparam longint  d = 0; // 64b
  localparam bit      e = 0; // 1b
endmodule
```

### Fail Example

```SystemVerilog
module A;
  localparam integer a = 0; // 32b
  localparam logic   b = 0; // 1b
  localparam reg     c = 0; // 1b
endmodule
```

### Explanation

In order to avoid subtle bugs related to 2-state vs 4-state types and
X-propagation, constants should be declared with an explicit 2-state type.
Separately, all synthesizable signals should be declared with an explicit
4-state type so that a simulation can detect unknown values (Xs).
For complex types such as nested packed structs, that means you need two
versions of each type: a 2-state version for constants, and a 4-state version
for signals.
The need for this rule stems from the fact that SystemVerilog includes the
concepts of both equivalence and partial equivalence, with interactions between
2-state and 4-state structure members which invite mismatching behavior
between simulation and synthesis.

The relevant quote about implicit conversion of packed structure members from
2-state to 4-state is found on page 140 of IEEE1800-2017:
If all data types within a packed structure are 2-state, the structure as a
whole is treated as a 2-state vector.
If any data type within a packed structure is 4-state, the structure as a whole
is treated as a 4-state vector.
If there are also 2-state members in the structure, there is an implicit
conversion from 4-state to 2-state when reading those members and from 2-state
to 4-state when writing them.

For constants of simple datatypes, it is trivial to visually check that their
values do not contain Xs or Zs.
However, for constants of more complex datatypes, e.g. nested packed
structures, the use of constant functions may infer Xs as (accidentally)
unassigned members will take their default values.
Default values are specified in IEEE1800-2017 Table 6-7.
This can be particularly subtle when a single member of a deeply nested packed
struct is wrongly declared with a 4-state type, e.g. `logic`, thus forcing all
other (previously 2-state) members to have a default value of `'X` instead of
the expected `'0`.

The equivalence operators ("case" equality/inequality) are written as 3
characters each (`===`, `!==`) and can only return false or true,
e.g. `4'b01XZ === 4'b01XZ` -> `1'b1` (true).
The partial equivalence operators ("logical" equality/inequality) are written
as 2 characters each (`==`, `!=`) and may return false, true, or unknown,
e.g. `4'b01XZ === 4'b01XZ` -> `1'bx` (unknown).

Let `w` be a 4-state signal which a systhesis tool will implement with a
collection of wires.
Let `c2` and `c4` be constants with 2-state and 4-state types respectively.
Without loss of generality, only the case/logical equality operators are
required to demonstrate troublesome expressions.

- `w === c2` Result may be false (`1'b0`), true (`1'b1`).
  If `w` contains any Xz or Zs, then the result is false (`1'b0`).
  This is *not* desired behavior as Xs in `w` are hidden and simulation is
  likely, but not certain, to mismatch synthesized hardware.
- `w === c4` Result may be false (`1'b0`), true (`1'b1`).
  If `w` contains any Xz or Zs, then the result is is true iff the constant
  `c4` has been defined with corresponding Xs and Zs.
  Comparison between unknown and unknown is all but certain, to mismatch
  synthesized hardware.
- `w == c2` Result may be false (`1'b0`), true (`1'b1`), or unknown (`1'bX`).
  If `w` contains any Xs or Zs, then the result is unknown.
  This is desired behavior as it sufficiently models synthesized physical
  hardware.
- `w == c4` Result may be false (`1'b0`), true (`1'b1`), or unknown (`1'bX`).
  If `c4` contains any Xs or Zs, then the result will always be unknown.
  While that may be noticed early in simulation, unwitting designers may be
  tempted to prevent X-propagation on the result, thus hiding any issues with
  Xs or Zs on `w`.

The use of 4-state constants with wildcard equality operators is a slightly
different usecase.
If wildcard equality operators are used with 4-state constants in your code,
this rule should be considered on a case-by-case basis.

See also:
  - **localparam_explicit_type** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.8 Variable declarations
  - 6.11 Integer data types
  - 7.2.1 Packed structures
  - 11.4.5 Equality operators
  - 11.4.6 Wildcard equality operators

NOTE: The reasoning behind this rule invites the creation of some new rules:
1. Check that members of a packed structure definition are either all 2-state
  or all 4-state.
2. Check for the use of case equality operators.
3. Check that functions are not declared with a 4-state type.


---
## `loop_variable_declaration`

### Hint

loop variable must be declared in loop

### Reason

the scope of variable should be minimized

### Pass Example

```SystemVerilog
module A;
initial begin
for(int i=0;i<10;i++) begin
end
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
initial begin
int i;
for(i=0;i<10;i++) begin
end
end
endmodule
```

### Explanation

TODO


---
## `lowercamelcase_interface`

### Hint

Interface name must begin with lowerCamelCase

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
interface fooBar; endinterface
```

### Fail Example

```SystemVerilog
interface FooBar; endinterface
```

### Explanation

TODO


---
## `lowercamelcase_module`

### Hint

Module name must begin with lowerCamelCase

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module fooBar; endmodule
```

### Fail Example

```SystemVerilog
module FooBar; endmodule
```

### Explanation

TODO


---
## `lowercamelcase_package`

### Hint

Package name must begin with lowerCamelCase

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
package fooBar; endpackage
```

### Fail Example

```SystemVerilog
package FooBar; endpackage
```

### Explanation

TODO


---
## `multiline_for_begin`

### Hint

Add `begin`/`end` around multi-line `for` statement.

### Reason

Without `begin`/`end`, the loop statement may be confusing.

### Pass Example

```SystemVerilog
module A;
always_comb begin
    for (int a=0; a<10; a++) begin
        a = 0;
    end
    for (int a=0; a<10; a++) a = 0;
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always_comb begin
    for (int a=0; a<10; a++)
        a = 0;
    for (int a=0; a<10; a++) a = 0;
end
endmodule
```

### Explanation

This rule is to help prevent a common class of coding mistake, where a future
maintainer attempts to add further statements to the loop, but accidentally
writes something different.

See also:
  - **multiline_if_begin** - Useful companion rule.
  - **style_indent** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.7 Loop statements


---
## `multiline_if_begin`

### Hint

Add `begin`/`end` around multi-line `if` statement.

### Reason

Without `begin`/`end`, the conditional statement may be confusing.

### Pass Example

```SystemVerilog
module A;
always_comb begin
    if (a) begin
        a = 0;
    end

    if (a) begin
        a = 0;
    end else if (a) begin
        a = 0;
    end

    if (a) begin
        a = 0;
    end else if (a) begin
        a = 0;
    end else begin
        a = 0;
    end

    if (a) a = 0;
    else if (a) a = 0;
    else a = 0;
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always_comb begin
    if (a)
        a = 0;

    if (a) begin
        a = 0;
    end else if (a)
        a = 0;

    if (a) begin
        a = 0;
    end else if (a) begin
        a = 0;
    end else
        a = 0;

    if (a) a = 0;
    else if (a) a = 0;
    else a = 0;
end
endmodule
```

### Explanation

This rule is to help prevent a common class of coding mistake, where a future
maintainer attempts to add further statements to the conditional block, but
accidentally writes something different.

See also:
  - **multiline_for_begin** - Useful companion rule.
  - **style_indent** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.4 Conditional if-else statement


---
## `non_ansi_module`

### Hint

module declaration must be ANSI-style

### Reason

non-ANSI-style has duplicated port declaration

### Pass Example

```SystemVerilog
module A(
    input  a,
    output b
);
endmodule
```

### Fail Example

```SystemVerilog
module A(
    a,
    b
);
input  a;
output b;
endmodule
```

### Explanation

TODO


---
## `non_blocking_assignment_in_always_comb`

### Hint

non-blocking assignment is forbidden in`always_comb`

### Reason

non-blocking assignment in `always_comb` causes elaboration error

### Pass Example

```SystemVerilog
module A;
always_comb begin
    x = 0;
end
endmodule
```

### Fail Example

```SystemVerilog
module A;
always_comb begin
    x <= 0;
end
endmodule
```

### Explanation

TODO


---
## `output_with_var`

### Hint

`output` must have `var`

### Reason



### Pass Example

```SystemVerilog
module A (
    output var a
);
endmodule
```

### Fail Example

```SystemVerilog
module A (
    output logic a
);
endmodule
```

### Explanation

TODO


---
## `parameter_explicit_type`

### Hint

`parameter` must be have an explicit type

### Reason

parameter types show intent and improve readability

### Pass Example

```SystemVerilog
module A #(parameter int a = 0) ();
endmodule
```

### Fail Example

```SystemVerilog
module A #(parameter a = 0) ();
endmodule
```

### Explanation

TODO


---
## `parameter_in_package`

### Hint

`parameter` must be replaced to `localparam` in `package`

### Reason

some tools can't take `parameter` in `package`

### Pass Example

```SystemVerilog
package A;
localparam A = 1;
endpackage
```

### Fail Example

```SystemVerilog
package A;
parameter A = 1;
endpackage
```

### Explanation

TODO


---
## `parameter_type_twostate`

### Hint

`parameter` must be have a twostate type

### Reason

design constants should not contain X or Z bits.

### Pass Example

```SystemVerilog
module A #(
  parameter byte     a = 0, // 8b
  parameter shortint b = 0, // 16b
  parameter int      c = 0, // 32b
  parameter longint  d = 0, // 64b
  parameter bit      e = 0  // 1b
) ();
endmodule
```

### Fail Example

```SystemVerilog
module A #(
  parameter integer a = 0, // 32b
  parameter logic   b = 0, // 1b
  parameter reg     c = 0  // 1b
) ();
endmodule
```

### Explanation

TODO


---
## `prefix_inout`

### Hint

`inout` must have prefix "b_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module M
( inout var b_foo
, input var logic [FOO-1:0] b_bar
);
endmodule
```

### Fail Example

```SystemVerilog
module M
( inout var foo
, inout var logic [FOO-1:0] bar
);
endmodule
```

### Explanation

TODO


---
## `prefix_input`

### Hint

`input` must have prefix "i_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module M
( input var i_foo
, input var logic [FOO-1:0] i_bar
);
endmodule
```

### Fail Example

```SystemVerilog
module M
( input var foo
, input var logic [FOO-1:0] bar
);
endmodule
```

### Explanation

TODO


---
## `prefix_instance`

### Hint

Module instance must have prefix "u_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module A;
Foo #() u_foo (a, b, c);
endmodule
```

### Fail Example

```SystemVerilog
module A;
Foo #() foo (a, b, c);
endmodule
```

### Explanation

TODO


---
## `prefix_interface`

### Hint

`interface` name must have prefix "ifc_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
interface ifc_withPrefix; endinterface
```

### Fail Example

```SystemVerilog
interface noPrefix; endinterface
```

### Explanation

TODO


---
## `prefix_module`

### Hint

`module` name must have prefix "mod_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module mod_withPrefix; // Module identifier of declaration has prefix.
  M #(.A(1)) u_M (.a); // Module identifier of instance doesn't require prefix.
endmodule
```

### Fail Example

```SystemVerilog
module noPrefix; // Module identifier of declaration should have prefix.
endmodule
```

### Explanation

TODO


---
## `prefix_output`

### Hint

`output` must have prefix "o_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module M
( output var o_foo
, output var logic [FOO-1:0] o_bar
);
endmodule
```

### Fail Example

```SystemVerilog
module M
( output var foo
, output var logic [FOO-1:0] bar
);
endmodule
```

### Explanation

TODO


---
## `prefix_package`

### Hint

`package` name must have prefix "pkg_"

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
package pkg_withPrefix; endpackage
```

### Fail Example

```SystemVerilog
package noPrefix; endpackage
```

### Explanation

TODO


---
## `priority_keyword`

### Hint

`priority` is forbidden

### Reason

this causes mismatch between simulation and synthesis

### Pass Example

```SystemVerilog
module A();
initial begin
    case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Fail Example

```SystemVerilog
module A();
initial begin
    priority case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Explanation

TODO


---
## `sequential_block_in_always_comb`

### Hint

Keywords `begin` and `end` are forbidden within `always_comb`.

### Reason

Sequential blocks within `always_comb` introduce sequential dependencies.

### Pass Example

```SystemVerilog
module a;
  always_comb
    e = z;

  always_comb
    if (foo) f = z;
    else     f = z;

  always_comb
    case (foo)
      one:     g = z;
      two:     g = z;
      default: g = z;
    endcase
endmodule
```

### Fail Example

```SystemVerilog
module a;
  always_comb begin
    a = z;
  end

  always_comb
    if (bar) begin
      b = z;
    end

  always_comb
    if (bar) c = z;
    else begin
      c = z;
    end

  always_comb
    case (bar)
      one: begin
        d = z;
      end
      two: d = z;
      default: d = z;
    endcase
endmodule
```

### Explanation

This rule has two purposes:
1. Prevent mismatches between simulation and synthesis.
2. Avoid unnecessarily restricting the simulator's scheduler.

An `always_comb` block is scheduled for execution whenever any of the RHS
variables (or nets) change value, which can lead to unnecessary sequential
dependencies.
For example, the following block is requires that the "expensive" (in terms
of CPU time) function must be called to update `a` whenever `z` changes value,
in addition to whenever `y` changes value.
```systemverilog
always_comb begin
  a = expensive(y);
  b = z;
end
```

The above example can be reformed to allow the simulator more flexibility in
how it schedules processes.
Logical equivalence is maintained, and a synthesis tool will interpret these
examples equivalently.
Note that continuous assignment (using `assign`) is not sensitive to changes in
`y` because functions are not transparent.
```systemverilog
always_comb a = expensive(y);
assign b = z;
```

This rule is intended for synthesisable code only, not testbench code.
Testbenches often necessarily rely on sequential dependencies, but a synthesis
tool for digital synchronous logic will produce a netlist without sequential
dependencies.
That can lead to a mismatch between simulation and synthesis.

See also:
  - **style_indent** - Useful companion rule.
  - **sequential_block_in_always_ff** - Similar rule, different purpose.
  - **sequential_block_in_always_latch** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:
  - 4.6 Determinisim
  - 9.2.2.2 Combinational logic always_comb procedure
  - 9.3.1 Sequential blocks
  - 10.3 Continuous assignments
  - 10.4 Procedural assignments


---
## `sequential_block_in_always_ff`

### Hint

Keywords `begin` and `end` are forbidden within `always_ff`.

### Reason

Sequential blocks within `always_ff` may encourage overly-complex code.

### Pass Example

```SystemVerilog
module a;
  always_ff @(posedge clk)
    d <= z;

  always_ff @(posedge clk)
    if (foo) e <= z;

  always_ff @(posedge clk)
    if (foo) f <= z;
    else     f <= z;

  always_ff @(posedge clk)
    case (foo)
      one:     g <= z;
      two:     g <= z;
      default: g <= z;
    endcase
endmodule
```

### Fail Example

```SystemVerilog
module a;
  always_ff @(posedge clk) begin
    a <= z;
  end

  always_ff @(posedge clk)
    if (bar) begin
      b <= z;
    end

  always_ff @(posedge clk)
    if (bar) c <= z;
    else begin
      c <= z;
    end

  always_ff @(posedge clk)
    case (bar)
      one: begin
        d <= z;
      end
      two: d <= z;
      default: d <= z;
    endcase
endmodule
```

### Explanation

The consequences/purposes of this rule are perhaps subtle, particulaly in how
it works with companion rules **default_nettype_none**, **explicit_case_default**,
**explicit_if_else**, **style_indent**, and a guideline to avoid `for` within
`always_ff`.

In conjunction with these companion rules and guidelines, a nice consequence is
that editing code after the fact is "safe", i.e. not error prone.
Without `begin`/`end` adding another statement to a single-statement conditional
block may be error prone.
This is why coding styles for C-style languages often forbid writing
`if (a) foo;`, instead requiring `if (a) { foo; }` - because it's easy to forget
to add braces with an additional statement like `if (a) { foo; bar; }`.
While a simple rule is to require the use of `begin` and `end` (or `{` and `}`),
this introduces visual noise.
The goal is to guard programmers from making a simple and easy mistake.
This rule, in conjunction with the companion rules, achieves the same goal using
a different approach, in addition to providing other nice properties.

With a sequential block (marked by `begin` and `end`) you can assign to multiple
signals in a leaf conditon which can easily result in difficult-to-comprehend
logic, e.g.:
```systemverilog
always_ff @(posedge clk) begin
  if (cond) begin
    foo_q <= foo_d;       // Block was originally written for foo.
    bar_q <= bar_d;       // This was added later.
  end
  bar_q <= bar_d;         // What happens to bar_q?
end
```
By forbidding sequential blocks, you enforce that only one signal is assigned to
per leaf condition.
A nice consequence is that exactly one signal is updated each evaluation of the
`always_ff` block is evaluated.
IEEE1800-2017 specifies that if a signal is assigned to in an `always_ff` block,
then it shall not be assigned to by any other block (compile error).

An example with multiple signals in the `always_ff` is a ping-pong buffer (AKA
shunt buffer, storage of a 2-entry fifo).
Due to the construction, you can be sure that you never update both entries at
the same time, except when that is clearly explicit.
```systemverilog
  // Enforced exclusive updates, with reset and clockgate.
  always_ff @(posedge clk)
    if (rst)
      {ping_q, pong_q} <= '0; // Assignment to multiple signals is explicit.
    else if (clkgate)
      if (foo) ping_q <= foo;
      else     pong_q <= foo;
    else // Optional explicit else.
      {ping_q, pong_q} <= {ping_q, pong_q};
```

Another example with multiple signals is an address decoder.
Due to the construction, you can be sure that you aren't accidentally updating
multiple registers on a write to one address.
```systemverilog
  // Enforced exclusivity of address decode.
  always_ff @(posedge clk)
    if (write)
      case (addr)
        123:        red_q   <= foo;
        456:        blue_q  <= foo;
        789:        green_q <= foo;
        default:    black_q <= foo; // Optional explicit default.
      endcase
```

When you don't need those exclusivity properties, only one signal should be
updated per `always_ff`.
That ensures that the code doesn't get too deep/complex/unintuitive and
drawing a logical diagram is straightforward.
This is the expected form for most signals.
```systemverilog
  always_ff @(posedge clk)
    if (rst)          ctrl_q <= '0;
    else if (clkgate) ctrl_q <= ctrl_d;
    else              ctrl_q <= ctrl_q; // Optional explicit else.
```

See also:
  - **default_nettype_none** - Useful companion rule.
  - **explicit_case_default** - Useful companion rule.
  - **explicit_if_else** - Useful companion rule.
  - **style_indent** - Useful companion rule.
  - **sequential_block_in_always_comb** - Similar rule, different purpose.
  - **sequential_block_in_always_latch** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:
  - 4.6 Determinisim
  - 9.2.2.4 Sequential logic always_ff procedure
  - 9.3.1 Sequential blocks
  - 9.4.2 Event control
  - 12.4 Conditional if-else statement
  - 12.5 Case statement
  - 12.7 Loop statements


---
## `sequential_block_in_always_latch`

### Hint

Keywords `begin` and `end` are forbidden within `always_latch`.

### Reason

Sequential blocks within `always_latch` may encourage overly-complex code.

### Pass Example

```SystemVerilog
module a;
  always_latch
    if (foo) e <= z;

  always_latch
    if (foo) f <= z;
    else     f <= z;

  always_latch
    case (foo)
      one:     g <= z;
      two:     g <= z;
      default: g <= z;
    endcase
endmodule
```

### Fail Example

```SystemVerilog
module a;
  always_latch begin
    a <= z;
  end

  always_latch
    if (bar) begin
      b <= z;
    end

  always_latch
    if (bar) c <= z;
    else begin
      c <= z;
    end

  always_latch
    case (bar)
      one: begin
        d <= z;
      end
      two: d <= z;
      default: d <= z;
    endcase
endmodule
```

### Explanation

The explanation of **sequential_block_in_always_ff**, and much of the explanation
of **sequential_block_in_always_comb**, also applies to this rule.
Main points are that avoiding `begin`/`end` helps protect the programmer against
simple mistakes, provides exclusivity properties by construction, and avoids
restricting simulator scheduling decisions.

See also:
  - **default_nettype_none** - Useful companion rule.
  - **explicit_case_default** - Useful companion rule.
  - **explicit_if_else** - Useful companion rule.
  - **style_indent** - Useful companion rule.
  - **sequential_block_in_always_comb** - Similar rule, different purpose.
  - **sequential_block_in_always_ff** - Similar rule, different purpose.

The most relevant clauses of IEEE1800-2017 are:
  - 4.6 Determinisim
  - 9.2.2.3 Latched logic always_latch procedure
  - 9.3.1 Sequential blocks
  - 9.4.2 Event control
  - 12.4 Conditional if-else statement
  - 12.5 Case statement
  - 12.7 Loop statements


---
## `style_commaleading`

### Hint

comma should be followed by a single space (comma-leading format)

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module M
#(bit FOO = 1 // comment
, int BAR = 2 /* comment */
, bit [31:0] BAZ = 2
)
( input  var logic i_abc // comment
, output var logic o_ghi /* comment */
);
  assign {foo, bar} =
    { i_abc
    , 12'h345
    , b_def     // comment
    , 16'h3456  /* comment */
    };
  assign singleline2D = {{foo, bar}, {foo, bar}, {foo, bar}};
  function foo
  ( input a
  , input b
  );
  endfunction
endmodule
```

### Fail Example

```SystemVerilog
module M
#( bit FOO = 1 // space after `#(` causes misalignment
, int BAR = 2
,  bit [31:0] BAZ = 2 // too many spaces after comma
)
(input  var logic i_abc // missing space after `(`
,output var logic o_ghi // missing space after comma
);
  assign {foo, bar} = { // brace not followed by a single space
      i_abc
    ,12'h345 // missing space after `(`
    ,  b_def // too many spaces after comma
    };
  function foo
  (input a // missing space after `(`
  ,  input b // too many spaces after comma
  );
  endfunction
endmodule
```

### Explanation

TODO


---
## `style_indent`

### Hint

newline should be followed by a multiple of 2 spaces

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module M;
  if (a)
    a = 0;
  else
    a = 1;
  // comment
/*
  comment
*/
endmodule
```

### Fail Example

```SystemVerilog
module M;
 if (a)
   a = 0;
   else
     a = 1;
   // comment
/*
 comment
   */
endmodule
```

### Explanation

TODO


---
## `style_keyword_0or1space`

### Hint

keyword should be followed by a symbol or exactly 1 space

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module A;
  function foo();
    if (a)
      return; // semicolon immediately after `return`.
    else
      return a; // 1 space then expression after `return`.
  endfunction
endmodule

```

### Fail Example

```SystemVerilog
module A;
  function foo();
    if (a)
      return  ; // multiple spaces after `return`.
  endfunction
endmodule

```

### Explanation

TODO


---
## `style_keyword_0space`

### Hint

keyword should be followed by no space before symbol

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module A;
  always_comb
    case (a)
      123:
        b = c;
      default: // no space between `default` and colon.
        b = d;
    endcase
  function foo ();
    for (;;)
      if (a) break; // no space between `break` and semicolon.
  endfunction
endmodule
```

### Fail Example

```SystemVerilog
module A;
  always_comb
    case (a)
      123:
        b = c;
      default : // space between `default` and colon.
        b = d;
    endcase
  function foo ();
    for (;;)
      if (a) break  ; // spaces between `break` and semicolon.
  endfunction
endmodule
```

### Explanation

TODO


---
## `style_keyword_1or2space`

### Hint

keyword should be followed by exactly 1 or 2 spaces

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module M (
  input a,
  inout b,  // 1 space after `input` or `inout` keywords
  output c, // makes port identifiers unaligned.

  input  d,
  inout  e, // 2 spaces after `input` or `inout` keywords
  output f  // makes port identifiers aligned.
);
endmodule
```

### Fail Example

```SystemVerilog
module M (
  input   a,
  inout   b   // multiple spaces after `input` or `inout` keywords
);
endmodule
```

### Explanation

TODO


---
## `style_keyword_1space`

### Hint

keyword should be followed by a single space

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module M;                   // 1 space after `module`.
  for (i = 0; i < 5; i++)   // 1 space after `for`.
    assign foo = bar;       // 1 space after `assign`.
  always_ff @(posedge clk)  // 1 space after `always_ff`.
    if (a)                  // 1 space after `if`.
      case (a)              // 1 space after `case`.
        1: foo <= bar;
      endcase
endmodule
```

### Fail Example

```SystemVerilog
module  M;                  // multiple spaces after `module`.
  for(i = 0; i < 5; i++)    // no spaces after `for`.
    assign  foo = bar;      // multiple spaces after `assign`.
  always_ff@(posedge clk)   // no spaces after `always_ff`.
    if  (a)                 // multiple spaces after `if`.
      case(a)               // no spaces after `case`.
        1: foo <= bar;
      endcase
endmodule
```

### Explanation

TODO


---
## `style_keyword_construct`

### Hint

keyword should be followed by newline or exactly 1 space

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module A;
  always_comb a = b;  // 1 space after `always_comb`.
  initial begin       // 1 space after `initial`.
    foo = bar;
  end
  always_latch
    if (a) b = c;     // newline after `always_latch`.
    else d = e;       // 1 space after `else`.
  final // 1 space then comment after `final`.
    foo = bar;
endmodule

```

### Fail Example

```SystemVerilog
module A;
  always_comb   a = b;  // multiple spaces after `always_comb`.
  initial     begin       // multiple spaces after `initial`.
    foo = bar;
  end
  always_latch
    if (a) b = c;
    else      d = e;  // multiple spaces after `else`.
  final  // multiple spaces then comment after `final`.
    foo = bar;
endmodule

```

### Explanation

TODO


---
## `style_keyword_datatype`

### Hint

keyword should be followed by a single space

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module M;
  localparam bit A = 0;   // 1 space after `bit`.
  localparam int B = 0;   // 1 space after `int`.
  logic a;                // 1 space after `logic`.
  reg b;                  // 1 space after `reg`.
  wire b;                 // 1 space after `wire`.
endmodule
```

### Fail Example

```SystemVerilog
module M;
  localparam bit  A = 0;  // multiple spaces after `bit`.
  localparam int
    B = 0;                // newline after `int`.
  logic // foo
    a;                    // single-line comment after `logic`.
  reg /* bar */ b;        // multi-line after `reg`.
  wire        c;          // multiple spaces after `wire`.
endmodule
```

### Explanation

TODO


---
## `style_keyword_end`

### Hint

keyword should be followed by newline, colon, or exactly 1 space

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module A ();
  initial begin
    if (foo) begin: l_foo
      a = b;
    end: l_foo           // colon immediately after `end`.

    if (foo) begin
      a = c;
    end else begin       // 1 space after `end`.
      a = d;
    end
//  ^^^ newline after `end`.
  end // 1 space then comment after `end`.
endmodule

```

### Fail Example

```SystemVerilog
module A;
  initial begin
    if (foo) begin: l_foo
      a = b;
    end   : l_foo           // spaces between `end` and colon.

    if (foo) begin
      a = c;
    end   else begin       // multiple spaces after `end`.
      a = d;
    end
  end   // multiple spaces then comment after `end`.
endmodule

```

### Explanation

TODO


---
## `style_keyword_maybelabel`

### Hint

keyword should be followed by newline or colon, not spaces

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module A;
endmodule: A // colon immediately after `endmodule`
package A;
    function foo();
    endfunction
//  ^^^^^^^^^^^ newline after `endfunction`
endpackage // 1 space then comment after `endpackage`

```

### Fail Example

```SystemVerilog
module A;
endmodule  : A // spaces immediately after `endmodule`
package A;
endpackage  // multiple spaces then comment after `endpackage`
interface A;
endinterface interface B; // space instead of newline after `endinterface`
endinterface
```

### Explanation

TODO


---
## `style_keyword_newline`

### Hint

keyword should be followed by a newline

### Reason

consistent style enhances readability

### Pass Example

```SystemVerilog
module A;
  generate
    case (foo)
      123: a = b;
    endcase
//  ^^^^^^^ newline after `endcase`
  endgenerate // 1 space then comment after `endgenerate`
endmodule

```

### Fail Example

```SystemVerilog
module A;
  generate
    case (foo)
      123: a = b;
    endcase if (foo) a = b; // no newline after `endcase`
  endgenerate   // multiple spaces then comment after `endgenerate`
endmodule

```

### Explanation

TODO


---
## `tab_character`

### Hint

tab character is forbidden

### Reason

may cause misalignment depending on editor setting

### Pass Example

```SystemVerilog
module A();
  logic a;
endmodule
```

### Fail Example

```SystemVerilog
module A();
	logic a;
endmodule
```

### Explanation

TODO


---
## `unique0_keyword`

### Hint

`unique0` is forbidden

### Reason

this causes mismatch between simulation and synthesis

### Pass Example

```SystemVerilog
module A();
initial begin
    case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Fail Example

```SystemVerilog
module A();
initial begin
    unique0 case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Explanation

TODO


---
## `unique_keyword`

### Hint

`unique` is forbidden

### Reason

this causes mismatch between simulation and synthesis

### Pass Example

```SystemVerilog
module A();
initial begin
    case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Fail Example

```SystemVerilog
module A();
initial begin
    unique case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Explanation

TODO


---
## `uppercamelcase_interface`

### Hint

Interface name must begin with UpperCamelCase

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
interface FooBar; endinterface
```

### Fail Example

```SystemVerilog
interface fooBar; endinterface
```

### Explanation

TODO


---
## `uppercamelcase_module`

### Hint

Module name must begin with UpperCamelCase

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
module FooBar; endmodule
```

### Fail Example

```SystemVerilog
module fooBar; endmodule
```

### Explanation

TODO


---
## `uppercamelcase_package`

### Hint

Package name must begin with UpperCamelCase

### Reason

Naming convention simplifies audit.

### Pass Example

```SystemVerilog
package FooBar; endpackage
```

### Fail Example

```SystemVerilog
package fooBar; endpackage
```

### Explanation

TODO


---
## `wire_reg`

### Hint

`wire`/`reg` must be replaced to `logic`/`tri`

### Reason

`logic` can detect multi-drive

### Pass Example

```SystemVerilog
module A;
logic a;
logic b;
endmodule
```

### Fail Example

```SystemVerilog
module A;
wire a;
reg b;
endmodule
```

### Explanation

TODO


