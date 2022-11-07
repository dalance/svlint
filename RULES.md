
# Introduction

## About This Document

This document is generated from the Markdown files in `md/*.md`, the rules'
source code (`svlint/src/rules/*.rs`), and their testcases
(`testcases/(fail|pass)/*.sv`) using the `mdgen` utility.

## Purpose of Lint Checks

The authors of any works must consider their audience, particularly in how
different sections of the audience will interpret the works.
For example, an author of childrens books has two main sections of audience
(children, and their adult parents) so they might aim to please both sections
at once; Children with simple storylines and colorful pictures; Parents with
cultural references and subtle innuendo.
Authors writing in SystemVerilog also have two main sections of audience which
they should aim to please: 1) other silicon engineers, 2) mechanical tools.
Although the differences between human and mechanical readers are significant,
both must be satisfied for the text to be nice/enjoyable to work with.
While a simulation tool doesn't care about whitespace, indentation, or
thoughtful comments, your human colleagues will dread working with messy code
(rewiewing, modifying, building upon outputs, etc.), which ultimately wastes
their time, money, and sanity.
Human readers will usually be polite about sub-par work, but tools are much more
direct, simply spitting back at you with warning messages and an outright
refusal to work if you dare to mis-spell a variable name.

There are two main classes of rule for helping human readers:
1. Rules which codify naming conventions.
2. Rules which codify style/formatting conventions.

Naming conventions help a human reader to take in large amounts of detailed
information (e.g. from netlists, timing reports) by allowing the reader to
predict the function of a signal from its name, and predict part of a signal's
name from its function.
For example, a common convention is: "All signals inferring the output of a
flip-flop must be suffixed with `_q`."
If an engineer reads a synthesized netlist and sees a flip-flop cell named
without the `_q` suffix, there may be a coding error, so further checks
with the author are required.
On the frontend, a reader can quickly scan a SystemVerilog file to check that
signals have the `_q` suffix if (and only if) they are driven in `always_ff`
processes.
Other naming conventions are useful for helping readers follow the direction of
signals through ports, find files quickly in a large filesystem, find
appropriate files from hierarchical paths in a netlist, and more.

Style conventions also help a human reader to quickly and efficiently
comprehend large bodies of code.
Indeed, that is exactly what a reader wants to do when they're working with
code written by other people, often complete strangers.
The reader simply wishes to open the file, extract the necessary information,
close the file, and get on with their life.
Unlike mechanical tools, people process code visually (by translating their
view of the screen into a mental model) and any noise which obscures the useful
information will require extra mental effort to process.
When code is written with consistent and regular whitespace, the important
details like operators and identifiers are easily extracted.
In contrast, when little attention is paid to indentation or spaces around
keywords, operators, or identifers, the readers must waste their energy
performing a mental noise reduction.
Two notable style conventions help with a change-review process, i.e. comparing
multiple versions of a file, rather than reading one version:
- Line length limited to a fixed number of characters, usually 80.
  - Excessively long lines may indicate problems with a program's logic.
  - Excessively long lines prevent viewing differences side-by-side.
  - Side-by-side reading is awkward when sideways scrolling is involved.
  - Code which is printed on paper cannot be scrolled sideways, and soft-wrap
    alternatives interrupt indentation.
- Trailing whitespace is forbidden.
  - Changes to trailing whitespace are not usually visible to human readers,
    but are found by version control tools.
  - Editors are often configured to remove trailing whitespace, resulting in
    unnecessary differences.
  - Git, a popular version control tool will (by default) warn against trailing
    whitespace with prominent markers specifically because of the unnecessary
    noise introduced to a repository's history.

Just as individual human readers have their own preferences (in language,
style, naming conventions, etc.), each tool has its own quirks and ways of
interpreting things, particularly when the language specification is not fully
explicit.
The most prominent example of tools' differences in interpretation of
SystemVerilog is between tools for simulation and tools for synthesis.
The SystemVerilog language is specifed in IEEE1800-2017, also known as the
Language Reference Manual (LRM).
The LRM is clear that the specification is written in terms of simulation, but
that some of its constructs may be synthesized into physical hardware.
This distinction is the basis for a class of functional rules which aim to
minimize the risk of introducing a mismatch between simulation and synthesis.
Another class of functional rules is those which check for datatypes and
constructs that avoid compiler checks for legacy compatibility.

## How It Works

This tool (svlint) works in a series of well-defined steps:
1. On startup, search for a configuration file or use a default configuration.
2. Examine the configuration to determine which rules should be enabled and
  load them into memory.
3. Parse a whole file for preprocessor constructs like `` `ifdef `` and
  `` `include ``.
4. Apply the preprocessor semantics to produce a source descrition text.
5. Parse the source description into a syntax tree.
  The grammatical structure of a syntax tree is described in IEEE1800-2017
  Annex A using Backus-Naur Form.
6. Iterate over each node of the syntax tree in order.
7. For each node, apply each rule independently.
8. If a rule detects an undesirable quality in the syntax tree, then return a
  failure, otherwise return a pass.


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

In each rule's explanation there is a "see also" list of other rules, each with
a short reason why it should be seen.
- "suggested companion" - Suggestions are given for rules which do not check
  semantics, i.e suggestions are for style and naming conventions only.
- "potential companion" - These are noted where the named rule is given
  primarily out of completeness, but their use may cause other issues.
  For example, **style_keyword_datatype** exists to ensure all SystemVerilog
  keywords are captured in the `style_keyword_*` set, but its use is not
  suggested because it is visually appealing (and common practice) to align
  the identifiers in declarations.
- "useful companion" - Enabling the named rule provides an additional set of
  properties which are useful for reasoning about the function and semantics of
  code which passes.
  For example, the conjunction of **localparam_type_twostate** and
  **localparam_explicit_type** allows for stronger confidence that the author
  has properly considered the type of each constant.
- "alternative" - The named rule *should* not be used in conjunction, i.e.
  enabling both rules is, at best, a waste compute power.
- "mutually exclusive alternative" - The named rule *can* not be used in
  conjunction, i.e. enabling both rules is nonsensical because a failure on one
  implies a pass on the other rule.


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
is undefined, so observed behavior is simulator-dependent.
As all examples in IEEE1800-2017 show, `always_ff` should only contain
non-blocking assignments in order for sampling and variable evaluation
to operate in a defined order.

Specifically, `always_ff` constructs should not contain blocking assignments:
- Blocking assignment operator, e.g. `foo = 123;`
- Increment/decrement operators, e.g. `foo++;`, `foo--;`.

See also:
  - **non_blocking_assignment_in_always_comb** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 4.9.3 Blocking assignment
  - 4.9.4 Non-blocking assignment
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

See also:
  - **explicit_case_default** - Useful companion rule.
  - **explicit_if_else** - Useful companion rule.
  - **legacy_always** - Useful companion rule.
  - **sequential_block_in_always_comb** - Useful companion rule.

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

SystemVerilog makes a distinction between variables (only 0 or 1 drivers)
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
The distinction between data storage elements and physical wires is therefore
made in using `always_comb`, `always_ff`, and (less commonly) `always_latch`
keywords.

Variables are preferred over nets for most digital logic for 2 reasons:
- Only 0 or 1 drivers allowed, so an accidental multi-driving is caught by
  a compile time error.
- Simulator performance (dependent on implemetation).
  Value can be found by lookup, rather than evaluation of drivers.
When `` `default_nettype none`` is used, all signals must be declared, thus
forcing the author to consider whether they mean a variable or a net.

See also:
  - **inout_with_tri** - Useful companion rule.
  - **input_with_var** - Useful companion rule.
  - **output_with_var** - Useful companion rule.

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
If no datatype is specified, then the default `int` (32b, 2-state) is implied.
For example, `enum {RED, BLACK} m; assign m = foo ? BLACK : RED;`
describes a multiplexor, but a simulator is unable to sufficiently model the
behavior of `m` when the value of `foo` is unknown.
A more appropriate declaration is
`typedef enum int {RED, BLACK} color; integer m;`.

Note: Comparison of 4-state variables against 2-state constants/enums *is*
appropriate, e.g. `logic a; a = (m == RED);`.

See also:
  - **localparam_explicit_type** - Useful companion rule.
  - **localparam_type_twostate** - Useful companion rule.
  - **parameter_explicit_type** - Useful companion rule.
  - **parameter_type_twostate** - Useful companion rule.

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
module M;
  always_comb
    case (x)
      1: y = 0;
      default: y = 0;
    endcase

  always_ff @(clk)
    case (x)
      1: y = 0;
      default: y = 0;
    endcase
endmodule
```

### Fail Example

```SystemVerilog
module M;
  always_comb
    case (x)
      1: y = 0; // Incompletely specified case implies memory.
    endcase

  always_ff @(clk) begin
    case (x)
      1: y = 0;
      default: y = 0; // Explicit default arm is good.
    endcase

    case (y)
      1: y = 0; // Implicit default arm.
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
module M;
  always_ff @(clk)
    if (x) y <= 0;
    else   y <= z;

  always_comb
    if (x) y = 0;
    else   y = z;
endmodule
```

### Fail Example

```SystemVerilog
module M;
  always_comb
    if (x) y = 0; // Incompletely specified condition implies memory.

  always_ff @(clk) begin
    if (a)
      b <= c;
    else // Explicit else clause is good.
      b <= d;

    if (b)
      c <= d; // Implicit else clause.
  end
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

See also:
  - **function_with_automatic** - Useful companion rule.

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

See also:
  - **function_same_as_system_function** - Useful companion rule.

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
## `genvar_declaration_in_loop`

### Hint

Declare `genvar` inside a loop generate construct.

### Reason

Minimized `genvar` scope makes code easier to read and review.

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

The specification of genvar declarations in IEEE1800-2017 is not
straightforward.
The formal syntax of `genvar_initialization` specified in Annex A.4.2 (Generated
instantiation) suggests that the `genvar` keyword is optional, but the second
sentence of Clause 27.5 declares that
"The loop index variable shall be declared in a genvar declaration prior to
its use in a loop generate scheme".
All 5 examples in Clause 27.4 (Loop generate constructs) declare the genvars
outside of the generate loops, and the formal syntax of `genvar_declaration` in
A.2.1.3 (Type declarations) is only applicable to declarations outside of
loop generate constructs.
That is, using syntax like `genvar i; for (i=0; ...)`.
However, several examples of declarations inside loop generate constructs are
present in other areas of the LRM like `for (genvar i=0; ...`:
- Clause 11.12 Let construct, example d, page 295.
- Clause 16.14.6.1 Arguments to procedural concurrent assertions, page 464.
- Clause 20.11 Elaboration system tasks, page 607.
- Clause 23.3.3.5 Unpacked array ports and arrays of instances, page 717.

Although it is not explicitly stated, a reasonable interpretation is that a
genvar declared inside a generate loop may only be used within that specific
loop generate construct, i.e. locally scoped.
This interpretation matches C99 (ISO/IEC 9899:1999), while a requirement for
the genvar to be declared outside would match ANSI C (ISO/IEC 9899:1990).
This rule checks that genvars are declared in a C99-like style so that the
identifier is declared beside its use which has several advantages:
- The purpose of the genvar is immediately clear, e.g. it is easy to read
  that the `i` in `for (genvar i=0; i < N_BITS; i++) ...` refers to a bit
  index.
  In contrast, `genvar j; ...many lines... for (j=0; j < N_BITS; j++) ...`
  requires the reader to keep `j` in their head for a longer time.
- Only one comment is necessary, rather than splitting or duplicating the
  information.
- When a future revision of your code removes a generate loop, the genvar
  declaration is implictly removed too, which avoids lingering useless and
  distracting statements.
- A subsequent generate loop cannot accidentally use a "leftover" genvar
  which is intended for use only by a previous generate loop.
  The LRM only requires that "A genvar shall not be referenced anywhere other
  than in a loop generate scheme.".

Given the lack of clarity in the LRM, it is unsurprising that some tools might
not support both ways of declaring genvars, so the related rule
**genvar_declaration_out_loop** assumes a stricter interpretation of the LRM
and checks that declarations must be separate from the generate loop syntax.

See also:
  - **genvar_declaration_out_loop** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.4 Loop generate constructs


---
## `genvar_declaration_out_loop`

### Hint

Declare `genvar` outside the loop generate construct.

### Reason

Some tools don't support `genvar` declarations inside loop generate constructs.

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

The specification of genvar declarations in IEEE1800-2017 is not
straightforward.
The formal syntax of `genvar_initialization` specified in Annex A.4.2 (Generated
instantiation) suggests that the `genvar` keyword is optional, but the second
sentence of Clause 27.5 declares that
"The loop index variable shall be declared in a genvar declaration prior to
its use in a loop generate scheme".
All 5 examples in Clause 27.4 (Loop generate constructs) declare the genvars
outside of the generate loops, and the formal syntax of `genvar_declaration` in
A.2.1.3 (Type declarations) is only applicable to declarations outside of
loop generate constructs.
That is, using syntax like `genvar i; for (i=0; ...)`.
However, several examples of declarations inside loop generate constructs are
present in other areas of the LRM like `for (genvar i=0; ...`:
- Clause 11.12 Let construct, example d, page 295.
- Clause 16.14.6.1 Arguments to procedural concurrent assertions, page 464.
- Clause 20.11 Elaboration system tasks, page 607.
- Clause 23.3.3.5 Unpacked array ports and arrays of instances, page 717.

This rule assumes a strict interpretation of the LRM and checks that
declarations must be separate from the generate loop syntax.

The related rule **genvar_declaration_in_loop** checks the opposite way because
C99-like declarations inside loop generate constructs can lead to code which is
easier to read and review.

See also:
  - **genvar_declaration_in_loop** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.4 Loop generate constructs


---
## `inout_with_tri`

### Hint

Specify `tri` datakind on `inout` ports.

### Reason

Explicit datakind of bi-directional ports should be consistent with input ports.

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

This check mandates that each `inout` port must be explicitly declared as a
tri-state net, rather than the default nettype.

The rules for determining port kind, datatype, and direction are specified in
IEEE1800-2017 Clause 23.2.2.3 and facilitate various shorthand notations which
are backwards compatible with the semantics of Verilog (IEEE1364-1995):
- `inout a` -> `inout tri logic a` The implicit datatype is `logic` and the
  default nettype is `tri` (without overriding via the `` `default_nettype ``
  compiler directive).
- `inout wire a` -> `inout tri logic a` Again, using the implicit datatype of
  `logic`;
  As `wire` is an alias for `tri`, this is equivalent to the above example.
- `inout logic a` -> `inout tri logic a` This time using an explicit datatype
  (`logic`) but relying on the default nettype for its datakind.
- `inout wire logic a` -> `inout tri logic a` Again, even with an explicit
  datatype (`logic`), the `wire` keyword is simply an alias for the datakind
  `tri`.

When the default nettype is overridden to none, i.e. with the compiler
directive `` `default_nettype none ``, inout ports require an explicit
datakind.

Although the semantics of `inout a` are equivalent in IEEE1364-1995, the intent
is not clearly described.
An author should use `inout` to declare ports which are driven both internally
and externally, but `input` to declare ports which should only be driven
externally.
In order to describe the intended bi-directional behavior, `inout` ports must
be declared with an explicit `tri` datakind.

See also:
  - **default_nettype_none** - Useful companion rule.
  - **input_with_var** - Suggested companion rule.
  - **output_with_var** - Suggested companion rule.
  - **prefix_inout** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.5 Nets and variables
  - 6.6 Net types
  - 22.8 default nettype
  - 23.2.2 Port declarations


---
## `input_with_var`

### Hint

Specify `var` datakind on `input` ports.

### Reason

Default datakind of input port is a tri-state net.

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


---
## `interface_port_with_modport`

### Hint

Specify the modport on the interface port.

### Reason

Without a modport, the interface port signals are all implictly `inout`.

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


---
## `keyword_forbidden_always`

### Hint

Use `always_comb`/`always_ff`/`always_latch` instead of `always`.

### Reason

General-purpose `always` cannot detect combinatorial/stateful (non-)blocking mistakes.

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

In Verilog (IEEE1364), there are two language constructs which can be used to
model combinatorial logic
1. Continuous assignment to `wire` signals is specified with the `assign`
  keyword.
2. `reg` signals are assigned to with an `always` block, which is evaluated
  whenever anything in the sensitivity list changes value.
The `always` keyword can also be used for modelling sequential logic by
including the edge of a signal in the sensitivity list.

The semantics of these keywords in SystemVerilog are compatible with Verilog,
but additional keywords (`always_comb`, `always_ff`, and `always_latch`) should
be used to clarify intent of digital designs.
The `always_*` keywords have slightly different semantics which are beneficial
for synthesizable designs:
1. `always_*` processes require compiler checks that any signals driven on the
  LHS are not driven by any other process, i.e. `always_*` cannot infer
  multi-driven or tri-state logic.
2. `always_comb` processes require a compiler check that the process does not
  infer state.
3. `always_ff` processes require a compiler check that the process does infer
  state.

This rule forbids the use of the general-purpose `always` keyword, thus forcing
authors of synthesizable design code to clarify their intent.
In verification code to be used in simulation only, a general-purpose `always`
process is a valid and useful way of scheduling events.
Therefore, this rule is intended only for synthesizable design code, not for
testbench code.

The alternative rule **level_sensitive_always** has similar reasoning but is
slightly relaxed, requiring that `always` blocks have an explicit sensitivity
list including an edge.
It is possible to construct a full-featured testbench where all `always` blocks
meet that requriment.
Therefore, it is appropriate to use **keyword_forbidden_always** on
synthesizable design code, but on verification code use
**level_sensitive_always** instead.

See also:
  - **level_sensitive_always** - Alternative rule.
  - **sequential_block_in_always_comb** - Useful companion rule.
  - **sequential_block_in_always_if** - Useful companion rule.
  - **sequential_block_in_always_latch** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 9.2.2 Always procedures
  - 9.5 Process execution threads


---
## `keyword_forbidden_generate`

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
  - **keyword_required_generate** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.3 Generate construct syntax


---
## `keyword_forbidden_priority`

### Hint

Remove `priority` keyword, perhaps replace with an assertion.

### Reason

Priority-case/if constructs may mismatch between simulation and synthesis.

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

The keyword `priority` may be used on `if`/`else` or `case` statements to
enable *violation checks* in simulation, and describe design intent for
synthesis.

A `priority if` statement without an explicit `else` clause will produce a
*violation report* in simulation if the implicit `else` condition is matched.
A `priority if` statement with an explicit `else` clause cannot produce a
violation report.
In synthesis, the `priority` keyword makes no difference to an `if`/`else`
statement, because the semantics of bare `if`/`else` statements already imply
priority logic.

A `priority case` statement without a `default` arm will produce a
violation report in simulation if the `default` condition is matched.
A `priority case` statement with an explicit `default` arm cannot produce a
violation report.
In synthesis, the `priority` keyword indicates that the designer has manually
checked that all of the possible cases are specified in the non-default arms.
This is equivalent to the use of the informal `full_case` directive comment
commonly seen in older Verilog code.

Violation checks only apply in simulation, not in synthesized hardware, which
allows for mismatches to occur.
For example, where violation reports are produced but ignored for whatever
reason, but the simulation does not otherwise check for the erroneous
condition, the synthesis tool may produce a netlist with the invalid assumption
that the condition cannot be met.

See also:
  - **case_default** - Useful companion rule.
  - **explicit_case_default** - Useful companion rule.
  - **keyword_forbidden_unique** - Useful companion rule.
  - **keyword_forbidden_unique0** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.4 Conditional if-else statement
  - 12.5 Case statement


---
## `keyword_forbidden_unique`

### Hint

Remove `unique` keyword, perhaps replace with an assertion.

### Reason

Unique-case/if constructs may mismatch between simulation and synthesis.

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

The keyword `unique` may be used on `if`/`else` or `case` statements to
enable *violation checks* in simulation, describe design intent for synthesis,
and change the semantics of condition priority.

A `unique if` statement without an explicit `else` clause will produce a
*violation report* in simulation if the implicit `else` condition is matched,
or more than one `if` conditions are matched.
A `unique if` statement with an explicit `else` clause will produce a violation
report when more than one of the `if` conditions are matched.
Thus, the conditions in a `unique if` statement may be evaluated in any order.
A `unique case` statement will produce a violation report if multiple arms
match the case expression.

In synthesis, the `unique` keyword on an `if`/`else` statement specifies that
priority logic (between the conditions) is not required - a significant change
in semantics vs a bare `if`/`else` statement.
Similarly, priority logic is not required between arms of a `unique case`
statement.
The `unique` keyword indicates that the designer has manually checked that
exactly 1 of the specified conditions must be met, so all conditions may be
safely calculated in parallel.
This is equivalent to the use of the informal `parallel_case` and `full_case`
directive comments commonly seen in older Verilog code.

In simulation, after finding a uniqueness violation in a `unique if`, the
simulator is not required to evaluate or compare the rest of the conditions.
However, in a `unique case`, all case item expressions must be evaluated even
once a matching arm is found.
These attributes mean that the presence of side effects, e.g. `$display()` or
`foo++`, may cause non-deterministic results.

Violation checks only apply in simulation, not in synthesized hardware, which
allows for mismatches to occur.
For example, where violation reports are produced but ignored for whatever
reason, but the simulation does not otherwise check for the erroneous
condition, the synthesis tool may produce a netlist with the invalid assumption
that the conditions can be safely evaluated in parallel.

See also:
  - **case_default** - Useful companion rule.
  - **explicit_case_default** - Useful companion rule.
  - **keyword_forbidden_priority** - Useful companion rule.
  - **keyword_forbidden_unique0** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.4 Conditional if-else statement
  - 12.5 Case statement


---
## `keyword_forbidden_unique0`

### Hint

Remove `unique0` keyword, perhaps replace with an assertion.

### Reason

Unique0-case/if constructs may mismatch between simulation and synthesis.

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

The keyword `unique0` may be used on `if`/`else` or `case` statements to
enable *violation checks* in simulation, describe design intent for synthesis,
and change the semantics of condition priority.

A `unique0 if` statement will produce a *violation report* in simulation if
more than one `if` condition is matched.
Thus, the conditions in a `unique0 if` statement may be evaluated in any order.
In synthesis, the `unique0` keyword specifies that priority logic (between the
conditions) is not required - a significant change in semantics vs a bare
`if`/`else` statement.

In synthesis, the `unique0` keyword on an `if`/`else` statement specifies that
priority logic (between the conditions) is not required - a significant change
in semantics vs a bare `if`/`else` statement.
Similarly, priority logic is not required between arms of a `unique0 case`
statement.
The `unique0` keyword indicates that the designer has manually checked that
exactly 0 or 1 of the specified conditions must be met, so all conditions may
be safely calculated in parallel.
This is equivalent to the use of the informal `parallel_case` and `full_case`
directive comments commonly seen in older Verilog code.

In simulation, after finding a uniqueness violation in a `unique0 if`, the
simulator is not required to evaluate or compare the rest of the conditions.
However, in a `unique0 case`, all case item expressions must be evaluated even
once a matching arm is found.
These attributes mean that the presence of side effects, e.g. `$display()` or
`foo++`, may cause non-deterministic results.

Violation checks only apply in simulation, not in synthesized hardware, which
allows for mismatches to occur.
For example, where violation reports are produced but ignored for whatever
reason, but the simulation does not otherwise check for the erroneous
condition, the synthesis tool may produce a netlist with the invalid assumption
that the conditions can be safely evaluated in parallel.

See also:
  - **case_default** - Useful companion rule.
  - **explicit_case_default** - Useful companion rule.
  - **keyword_forbidden_priority** - Useful companion rule.
  - **keyword_forbidden_unique** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.4 Conditional if-else statement
  - 12.5 Case statement


---
## `keyword_forbidden_wire_reg`

### Hint

Replace `wire` or `reg` keywords with `logic`, `tri` and/or `var`.

### Reason

Explicit datatype `logic` and/or datakind `var`/`tri` better describes intent.

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

The keywords `wire` and `reg` are present in SystemVerilog primarily for
backwards compatibility with Verilog (IEEE1364-1995).
In SystemVerilog, there are additional keywords, such as `logic` and `tri`
with more refined semantics to better express the programmer's intent.

The LRM covers the use of `wire`:
> The net types `wire` and `tri` shall be identical in their syntax and
> functions; two names are provided so that the name of a net can indicate the
> purpose of the net in that model.

The LRM covers the use of `reg`:
> The keyword `reg` does not always accurately describe user intent, as it
> could be perceived to imply a hardware register. The keyword `logic` is a
> more descriptive term. `logic` and `reg` denote the same type.

See also:
  - **default_nettype** - Useful companion rule.
  - **inout_with_tri** - Useful companion rule.
  - **input_with_var** - Useful companion rule.
  - **output_with_var** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.6.1 Wire and tri nets
  - 6.11.2 2-state (two-value) and 4-state (four-value) data types


---
## `keyword_required_generate`

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
  - **keyword_forbidden_generate** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.3 Generate construct syntax


---
## `level_sensitive_always`

### Hint

Replace level-sensitive `always` with `always_comb`.

### Reason

Level-sensitive `always` cannot detect combinatorial/stateful (non-)blocking mistakes.

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

In Verilog (IEEE1364), there are two language constructs which can be used to
model combinatorial logic
1. Continuous assignment to `wire` signals is specified with the `assign`
  keyword.
2. `reg` signals are assigned to with an `always` block, which is evaluated
  whenever anything in the sensitivity list changes value.
The `always` keyword can also be used for modelling sequential logic by
including the edge of a signal in the sensitivity list.

The semantics of these keywords in SystemVerilog are compatible with Verilog,
but additional keywords (`always_comb`, `always_ff`, and `always_latch`) should
be used to clarify intent of digital designs.
The `always_*` keywords have slightly different semantics which are beneficial
for synthesizable designs:
1. `always_*` processes require compiler checks that any signals driven on the
  LHS are not driven by any other process, i.e. `always_*` cannot infer
  multi-driven or tri-state logic.
2. `always_comb` processes require a compiler check that the process does not
  infer state.
3. `always_ff` processes require a compiler check that the process does infer
  state.

This rule requires that general-purpose `always` blocks have an explicit
sensitivity list which includes at least one edge, thus forcing the use of
`assign` or `always_comb` to specify combinatorial logic.
It is possible to construct a full-featured testbench where all `always` blocks
meet that requriment.
The alternative rule **keyword_forbidden_always** has similar reasoning but is
more strict, completely forbidding the use of general-purpose `always` blocks.
It is appropriate to use **keyword_forbidden_always** on synthesizable design
code, but on verification code use **level_sensitive_always** instead.

See also:
  - **keyword_forbidden_always** - Alternative rule.

The most relevant clauses of IEEE1800-2017 are:
  - 9.2.2 Always procedures
  - 9.5 Process execution threads


---
## `localparam_explicit_type`

### Hint

Provide an explicit type in `localparam` declaration.

### Reason

Explicit parameter types clarify intent and improve readability.

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

The type of a parameter is more fundmental to express intent than its value.
By analogy, asking a shopkeeper for "5 oranges" is more likely to be correctly
understood than simply asking for "5" without clarification.
This rule requires that authors consider and specify the type of each
`localparam` elaboration-time constant.
Explicit types help readers to understand exactly what effects the constant
might have, thus reducing the effort they need to expend reading how the
parameter is used.

Without an explicit type, a localparam will take a type compatible with its
constant expression.
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
  - **localparam_type_twostate** - Useful companion rule.
  - **parameter_explicit_type** - Useful companion rule.
  - **parameter_type_twostate** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.3 Value set
  - 6.11 Integer data types
  - 6.20.2 Value parameters
  - 6.20.4 Local parameters (localparam)


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
If all datatypes within a packed structure are 2-state, the structure as a
whole is treated as a 2-state vector.
If any datatype within a packed structure is 4-state, the structure as a whole
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
  - **parameter_explicit_type** - Useful companion rule.
  - **parameter_type_twostate** - Useful companion rule, equivalent reasoning.

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

Declare the loop variable within the loop, i.e. `for (int i`.

### Reason

Minimizing the variable's scope avoids common coding errors.

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

A loop variable may be declared either inside the loop, e.g.
`for (int i = 0; i < 5; i++)`, or outside the loop, e.g.
`int i; ... for (i = 0; i < 5; i++)`.
This rule mandates that the scope of a loop variable, e.g. `i`, is minimized to
avoid a common class of coding mistake where `i` is erroneously used outside
the loop.

See also:
  - **function_with_automatic** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 12.7 Loop statements


---
## `lowercamelcase_interface`

### Hint

Begin `interface` name with lowerCamelCase.

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
In Haskell, types/typeclasses must start with an uppercase letter, and
functions/variables must start with a lowercase letter.
This rule checks part of a related naming scheme where modules and interfaces
should start with a lowercase letter, and packages should start with an
uppercase letter.

See also:
  - **lowercamelcase_module** - Suggested companion rule.
  - **lowercamelcase_package** - Potential companion rule.
  - **prefix_interface** - Alternative rule.
  - **uppercamelcase_interface** - Mutually exclusive alternative rule.
  - **uppercamelcase_module** - Potential companion rule.
  - **uppercamelcase_package** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `lowercamelcase_module`

### Hint

Begin `module` name with lowerCamelCase.

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
In Haskell, types/typeclasses must start with an uppercase letter, and
functions/variables must start with a lowercase letter.
This rule checks part of a related naming scheme where modules and interfaces
should start with a lowercase letter, and packages should start with an
uppercase letter.

See also:
  - **lowercamelcase_interface** - Suggested companion rule.
  - **lowercamelcase_package** - Potential companion rule.
  - **prefix_module** - Alternative rule.
  - **uppercamelcase_interface** - Potential companion rule.
  - **uppercamelcase_module** - Mutually exclusive alternative rule.
  - **uppercamelcase_package** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `lowercamelcase_package`

### Hint

Begin `package` name with lowerCamelCase.

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
In Haskell, types/typeclasses must start with an uppercase letter, and
functions/variables must start with a lowercase letter.
This rule checks part of a related naming scheme where modules and interfaces
should start with an uppercase letter, and packages should start with an
lowercase letter.

See also:
  - **lowercamelcase_interface** - Potential companion rule.
  - **lowercamelcase_module** - Potential companion rule.
  - **prefix_package** - Alternative rule.
  - **uppercamelcase_interface** - Suggested companion rule.
  - **uppercamelcase_module** - Suggested companion rule.
  - **uppercamelcase_package** - Mutually exclusive alternative rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


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

Declare `module` header in ANSI style.

### Reason

Non-ANSI module headers are visually noisy and error-prone.

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

There are two ways to declare a module header in SystemVerilog:
1. ANSI style - newer, neater, more succinct, compatible with IEEE1364-2001.
2. non-ANSI style - additionally compatible with older Verilog (IEEE1364-1995).
Examples of both styles are given in IEEE1364-2001 (e.g. pages 180 vs 182) and
IEEE1800-2017 (e.g. pages 702 vs 700).

The non-ANSI style separates the declaration of ports, their direction, and
their datatype.
In addition to requiring more text, and visual noise, to convey the same
information, the non-ANSI style encourages simple coding mistakes where
essential attributes may be forgotten.
This rule requires that module headers are declared using the ANSI style.

See also:
  - None applicable

The most relevant clauses of IEEE1800-2017 are:
  - 23.2 Module definitions


---
## `non_blocking_assignment_in_always_comb`

### Hint

Remove non-blocking assignment in `always_comb`.

### Reason

Scheduling between blocking and non-blocking assignments is non-deterministic.

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

Simulator event ordering between blocking and non-blocking assignments
is undefined, so observed behavior is simulator-dependent.
This rule forbids the use of non-blocking assigments (using the `<=` operator)
in `always_comb` blocks.
Instead, use the blocking assignment operator `=`.

An excellent paper detailing the semantics of Verilog blocking and non-blocking
assignments is written by Clifford E Cummings and presented at SNUG-2000,
"Nonblocking Assignments in Verilog Synthesis, Coding Styles that Kill".

See also:
  - **blocking_assignment_in_always_ff** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 4.9.3 Blocking assignment
  - 4.9.4 Non-blocking assignment
  - 9.2.2.2 Combinational logic `always_comb` procedure
  - 9.4.2 Event control
  - 10.4.1 Blocking procedural assignments
  - 10.4.2 Nonblocking procedural assignments


---
## `output_with_var`

### Hint

Specify `var` datakind on `output` ports.

### Reason

Explicit datakind of output ports should be consistent with input ports.

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


---
## `parameter_explicit_type`

### Hint

Provide an explicit type in `parameter` declaration.

### Reason

Explicit parameter types clarify intent and improve readability.

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


---
## `parameter_in_package`

### Hint

Replace `parameter` keyword with `localparam`.

### Reason

In a package, `localparam` properly describes the non-overridable semantics.

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

In the context of a package, the `parameter` keyword is a synonym for the
`localparam` keyword.
This rule encourages the author to consider that the constant may not be
overridden and convey that explictly.

See also:
  - None applicable.

The most relevant clauses of IEEE1800-2017 are:
  - 6.20.4 Local parameters (localparam)
  - 26 Packages


---
## `parameter_type_twostate`

### Hint

Declare `parameter` with an explicit 2-state type.

### Reason

Design constants with Xs or Zs may cause simulation/synthesis mismatch.

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

The reasoning behind this rule is equivalent to that of
**localparam_type_twostate**.
Please see the explanation for **localparam_type_twostate**.

See also:
  - **localparam_explicit_type** - Useful companion rule.
  - **localparam_type_twostate** - Useful companion rule, equivalent reasoning.
  - **parameter_explicit_type** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - 6.8 Variable declarations
  - 6.11 Integer data types
  - 7.2.1 Packed structures
  - 11.4.5 Equality operators
  - 11.4.6 Wildcard equality operators


---
## `prefix_inout`

### Hint

Prefix `inout` port identifier with "b_".

### Reason

Port prefixes help readers to follow signals through modules.

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

There are 4 kinds of SystemVerilog port (`inout`, `input`, `output`, and `ref`),
though `ref` is not generally used for synthesisable code.
For a new reader, unfamiliar with a large module, it is useful to be able to 
distinguish at a glance between which signals are ports and internal ones.
This is especially useful for an integrator who needs to read and understand the
boundaries of many modules quickly and accurately.
To use a visual analogy, prefixing port names is like adding arrowheads to a
schematic - they're not essential, but they speed up comprehension.
This rule requires the prefix `b_` (configurable) on bi-directional signals,
i.e, ports declared with direction `inout`, which is also the default direction.

See also:
  - **prefix_input** - Suggested companion rule.
  - **prefix_instance** - Suggested companion rule.
  - **prefix_output** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `prefix_input`

### Hint

Prefix `input` port identifier with "i_".

### Reason

Port prefixes help readers to follow signals through modules.

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

There are 4 kinds of SystemVerilog port (`inout`, `input`, `output`, and `ref`),
though `ref` is not generally used for synthesisable code.
For a new reader, unfamiliar with a large module, it is useful to be able to 
distinguish at a glance between which signals are ports and internal ones.
This is especially useful for an integrator who needs to read and understand the
boundaries of many modules quickly and accurately.
To use a visual analogy, prefixing port names is like adding arrowheads to a
schematic - they're not essential, but they speed up comprehension.
This rule requires the prefix `i_` (configurable) on `input` signals.

See also:
  - **prefix_inout** - Suggested companion rule.
  - **prefix_instance** - Suggested companion rule.
  - **prefix_output** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `prefix_instance`

### Hint

Prefix instance identifier with "u_".

### Reason

Naming convention helps investigation using hierarchical paths.

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

This rule requires that instances of modules or interfaces are prefixed with
`u_` (configurable) which allows readers to quickly find instances and
connections of interest.
Prefixing instances also allows components of a hierarchical path to be easily
identified as modules/interfaces rather than generate blocks, which is
especially useful when reading netlists and synthesis reports.
The default value of `u_` comes from the historical use of `U` for the PCB
reference designator of an inseparable assembly or integrated-circuit package,
as standardized in IEEE315-1975.

See also:
  - **generate_case_with_label** - Suggested companion rule.
  - **generate_for_with_label** - Suggested companion rule.
  - **generate_if_with_label** - Suggested companion rule.
  - **prefix_inout** - Suggested companion rule.
  - **prefix_input** - Suggested companion rule.
  - **prefix_output** - Suggested companion rule.
  - <https://en.wikipedia.org/wiki/Reference_designator>

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `prefix_interface`

### Hint

Prefix `interface` identifier with "ifc_".

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
This rule requires that interface identifiers are declared with a prefix of
`ifc_` (configurable) which allows a reader to easily distinguish between
module and interface instances.

See also:
  - **lowercamelcase_interface** - Alternative rule.
  - **prefix_module** - Potential companion rule.
  - **prefix_package** - Suggested companion rule.
  - **uppercamelcase_interface** - Alternative rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `prefix_module`

### Hint

Prefix `module` identifier with "mod_".

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
This rule requires that module identifiers are declared with a prefix of `mod_`
(configurable) which allows a reader to easily distinguish between
module and interface instances.

See also:
  - **lowercamelcase_module** - Alternative rule.
  - **prefix_interface** - Suggested companion rule.
  - **prefix_package** - Suggested companion rule.
  - **uppercamelcase_module** - Alternative rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `prefix_output`

### Hint

Prefix `output` port identifier with "o_".

### Reason

Port prefixes help readers to follow signals through modules.

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

There are 4 kinds of SystemVerilog port (`inout`, `input`, `output`, and `ref`),
though `ref` is not generally used for synthesisable code.
For a new reader, unfamiliar with a large module, it is useful to be able to 
distinguish at a glance between which signals are ports and internal ones.
This is especially useful for an integrator who needs to read and understand the
boundaries of many modules quickly and accurately.
To use a visual analogy, prefixing port names is like adding arrowheads to a
schematic - they're not essential, but they speed up comprehension.
This rule requires the prefix `o_` (configurable) on `output` signals.

See also:
  - **prefix_inout** - Suggested companion rule.
  - **prefix_input** - Suggested companion rule.
  - **prefix_instance** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `prefix_package`

### Hint

Prefix `package` identifier with "pkg_".

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
This rule requires that package identifiers are declared with a prefix of
`pkg_` (configurable).
When used in conjunction with a file naming scheme like "There should be one
package declaration per file, and a package `pkg_foo` must be contained in a
file called `pkg_foo.sv`.", this aids a reader in browsing a source directory.

See also:
  - **lowercamelcase_package** - Alternative rule.
  - **prefix_interface** - Suggested companion rule.
  - **prefix_module** - Potential companion rule.
  - **uppercamelcase_package** - Alternative rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


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
By forbidding sequential blocks, you enforce that exactly signal is assigned to
per leaf condition.
A nice consequence is that exactly one signal is updated on each evaluation of
the `always_ff` block.
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

Follow each comma with a single space (comma-leading format).

### Reason

Consistent style enhances readability.

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

This rule is intended to enforce consistent formatting of comma-separated lists
such as parameter/signal port declarations, concatenations, assignment
patterns, and function arguments.
The rule is very simple: Each comma must be followed by exactly 1 space.

Comma-leading style is seen extensively in other languages, e.g. Haskell, and
lends itself well to SystemVerilog, as seen in the following examples.
```systemverilog
/* Module declaration without parameter ports.
*/
module Mod_A
  ( input  var logic i_abc // comment
  , inout  tri logic b_def /* comment */
  , output var logic o_ghi
  );
endmodule

/* Module declaration with parameter ports.
*/
module Mod_B
  #(int FOO = 1 // comment
  , bit BAR = 2 /* comment */
  , bit [31:0] BAZ = 2
  , parameter int BUZZ = 4
  )
  ( input  var logic i_abc // comment
  , inout  tri logic b_def /* comment */
  , output var logic o_ghi
  );


  /* Each item on its own line.
  - Short lines.
  - Every list indented to same level.
  - Single-line LHS can be any length without indent issue.
  */
  assign {foo, bar} =
    { i_abc
    , 12'h345
    , b_def     // comment
    , 16'h3456  /* comment */
    };


  /* Everything can fit on one line.
  - No space after opening parenthesis/bracket/brace.
  */
  assign singleline1D = {i_abc, 12'h345, b_def, 16'h3456};
  assign singleline2D = {{foo, bar}, {foo, bar}, {foo, bar}};

  /* Multi-dimensional concatenation with innermost array on one line.
  */
  assign matrix2D_A =
    { {elem21, elem20}
    , {elem11, elem10} // comment
    , {elem01, elem00} /* comment */
    };
  assign matrix3D_A =
    { { {elem211, elem210}
      , {elem201, elem200}
      }
    , { {elem111, elem110} // comment
      , {elem101, elem100} /* comment */
      }
    , { {elem011, elem010}
      , {elem001, elem000}
      }
    };

  /* Multi-dimensional concatenation with one element per line.
  */
  assign matrix2D_B =
    { { elem21
      , elem20_with_long_name
      }
    , { elem11 // comment
      , elem10 /* comment */
      }
    , { elem01_note_no_misalignment
      , elem00
      }
    };

  /* Module instance without parameter ports.
  */
  Mod_A u_instanceA
    ( .i_abc(foo) // comment
    , .b_def({bar, bar}) /* comment */
    , .o_ghi
    );

  /* Module instance with parameter ports.
  */
  Mod_B
    #(.FOO(1) // comment
    , .BAR(2) /* comment */
    , .BUZZ(2)
    ) u_instanceB
    ( .i_abc(foo) // comment
    , .b_def({bar, bar}) /* comment */
    , .o_ghi
    );

endmodule
```

See also:
  - **style_indent** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_indent`

### Hint

Follow each newline with an integer multiple of 2 spaces.

### Reason

Consistent indentation is essential for readability.

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

Consistent indentation is essential for efficient reading by your human
colleagues.
This rule simply checks that any newline (outside of string literals) is
followed by an integer multiple of 2 (configurable) space characters.

See also:
  - **tab_character** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_0or1space`

### Hint

Follow keyword with a symbol or exactly 1 space.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following the `return` keyword.
The `return` keyword can be used without an argument for void functions, in
which case there should be no space between the keyword and the following
symbol, i.e. `return;`.
The `return` keyword can also be used with an argument, in which case there
should be exactly 1 space between the keyword and the following identifier,
e.g. `return foo;`.

See also:
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_0space`

### Hint

Remove all whitespace between keyword and following symbol.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following these keywords:
`break`
, `continue`
, `default`
, `new`
, `null`
, `super`
, and `this`.
Uses of these keywords should never have any whitespace between the keyword and
the following symbol, e.g.
`break;`,
, `continue;`
, `default:`
, `new[5]`
, `(myexample == null)`
, or `super.foo`.

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_1or2space`

### Hint

Follow keyword with exactly 1 or 2 spaces.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following the `inout` and `input`
keywords.
These keywords specify the direction of signal ports, and are frequently used
alongside the `output` keyword which is 1 character longer.
The suggested companion rule **style_keyword_1space** checks that `output` is
followed by a single space, and this rule allows `inout`/`input` to be followed
by a single space too.
However, it is common and visually appealing to have port definitions
vertically aligned, so this rule also allows 2 following spaces, e.g:
```systemverilog
module foo
  ( input  var logic i_foo // aligned, 2 spaces
  , output var logic o_bar
  , inout tri logic b_baz // unaligned, 1 space
  );
endmodule
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_1space`

### Hint

Follow keyword with exactly 1 space.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following these keywords:
`accept_on`
, `alias`
, `always`
, `always_ff`
, `and`
, `assert`
, `assume`
, `automatic`
, `before`
, `bind`
, `bins`
, `binsof`
, `bit`
, `buf`
, `bufif0`
, `bufif1`
, `case`
, `casex`
, `casez`
, `cell`
, `checker`
, `class`
, `clocking`
, `cmos`
, `config`
, `const`
, `constraint`
, `context`
, `cover`
, `covergroup`
, `coverpoint`
, `cross`
, `deassign`
, `defparam`
, `design`
, `disable`
, `dist`
, `do`
, `edge`
, `enum`
, `eventually`
, `expect`
, `export`
, `extends`
, `extern`
, `first_match`
, `for`
, `force`
, `foreach`
, `forever`
, `forkjoin`
, `function`
, `genvar`
, `global`
, `highz0`
, `highz1`
, `if`
, `iff`
, `ifnone`
, `ignore_bins`
, `illegal_bins`
, `implements`
, `implies`
, `import`
, `incdir`
, `include`
, `inside`
, `instance`
, `interconnect`
, `interface`
, `intersect`
, `large`
, `let`
, `liblist`
, `library`
, `local`
, `localparam`
, `macromodule`
, `matches`
, `medium`
, `modport`
, `module`
, `nand`
, `negedge`
, `nettype`
, `nexttime`
, `nmos`
, `nor`
, `noshowcancelled`
, `not`
, `notif0`
, `notif1`
, `or`
, `output`
, `package`
, `packed`
, `parameter`
, `pmos`
, `posedge`
, `primitive`
, `priority`
, `program`
, `property`
, `protected`
, `pull0`
, `pull1`
, `pulldown`
, `pullup`
, `pulsestyle_ondetect`
, `pulsestyle_onevent`
, `pure`
, `rand`
, `randc`
, `randcase`
, `randsequence`
, `rcmos`
, `reject_on`
, `release`
, `repeat`
, `restrict`
, `rnmos`
, `rpmos`
, `rtran`
, `rtranif0`
, `rtranif1`
, `s_always`
, `s_eventually`
, `s_nexttime`
, `s_until`
, `s_until_with`
, `scalared`
, `sequence`
, `showcancelled`
, `small`
, `soft`
, `solve`
, `specparam`
, `static`
, `strong`
, `strong0`
, `strong1`
, `struct`
, `sync_accept_on`
, `sync_reject_on`
, `tagged`
, `task`
, `throughout`
, `timeprecision`
, `timeunit`
, `tran`
, `tranif0`
, `tranif1`
, `trireg`
, `type`
, `typedef`
, `union`
, `unique`
, `unique0`
, `until`
, `until_with`
, `untyped`
, `use`
, `var`
, `vectored`
, `virtual`
, `wait`
, `wait_order`
, `weak`
, `weak0`
, `weak1`
, `while`
, `wildcard`
, `with`
, `within`
, `xnor`
, and `xor`.
This rule covers the majority of SystemVerilog keywords, ensuring that they are
followed by a single space, e.g. `if (foo)`, `always_ff @(posedge clk)`,
or `typedef struct packed {`.

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_construct`

### Hint

Follow keyword with a newline or exactly 1 space.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following these keywords:
`always_comb`
, `always_latch`
, `assign`
, `else`
, `final`
, `generate`
, and `initial`.
These keyword open constucts and should always be followed by a newline,
exactly 1 space then another keyword/identifier, or exactly 1 space then a
comment, e.g:
```systemverilog
// Followed by 1 space then another keyword.
always_comb begin
  foo = '0;
  foo[0] = 5;
end

// Followed by 1 space then an identifier.
always_comb bar = 5;

// Followed by a newline.
always_comb
  if (x < y)
    z = 5;
  else // Followed by 1 space then this comment.
    z = 6;

// Assign to a concatenation.
assign // You could use `always_comb` instead.
  { foo
  , bar
  , baz[i][j][k]
  } = '0;
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_datatype`

### Hint

Follow datatype keyword with a symbol or exactly 1 space.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following these keywords:
`byte`
, `chandle`
, `event`
, `int`
, `integer`
, `logic`
, `longint`
, `real`
, `realtime`
, `ref`
, `reg`
, `shortint`
, `shortreal`
, `signed`
, `string`
, `supply0`
, `supply1`
, `time`
, `tri`
, `tri0`
, `tri1`
, `triand`
, `trior`
, `unsigned`
, `uwire`
, `void`
, `wand`
, `wire`
, and `wor`.
These keywords are used to declare the datatype of signals/variables (like
`logic foo`), and cast expressions (like `int'(foo)`).

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_end`

### Hint

Follow keyword with a colon, newline, or exactly 1 space.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following the `end` keyword.
The keyword `end` always be followed by a newline,
exactly 1 space then another keyword, a colon, or exactly 1 space then a
comment, e.g:
```systemverilog
// Followed by a newline.
if (FOO) begin
  ...
end

// Followed by 1 space then a keyword.
if (FOO) begin
  ...
end else ...

// Followed by a colon.
if (FOO) begin: l_foo
  ...
end: l_foo

// Followed by a comment.
if (FOO) begin // {{{ An opening fold marker.
  ...
end // }}} A closing fold marker.
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_maybelabel`

### Hint

Follow keyword with a colon, newline, or exactly 1 space plus comment.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following these keywords:
`begin`
, `endchecker`
, `endclass`
, `endclocking`
, `endconfig`
, `endfunction`
, `endgroup`
, `endinterface`
, `endmodule`
, `endpackage`
, `endprimitive`
, `endprogram`
, `endproperty`
, `endsequence`
, `endtask`
, `fork`
, `join`
, `join_any`
, and `join_none`.
These keywords are used to delimit code blocks and should always be followed by
a colon, a newline, or exactly 1 space then a comment, e.g:
```systemverilog
if (FOO) begin: l_foo // Followed by a colon.
  ...
end

module top;
  ...
endmodule: top  // Followed by a colon.

// Followed by a newline.
if (FOO) begin
  ...
end

if (FOO) begin // Followed by a comment.
  ...
end
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `style_keyword_newline`

### Hint

Follow keyword with a newline or exactly 1 space plus comment.

### Reason

Consistent use of whitespace enhances readability by reducing visual noise.

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

This rule checks the whitespace immediately following these keywords:
, `endcase`
, `endgenerate`
, `endspecify`
, `endtable`
, `specify`
, and `table`.
These keywords are used to delimit code blocks and should always be followed by
a newline or exactly 1 space then a comment, e.g:
```systemverilog
case (FOO)
  ...
endcase // Followed by a comment.

// Followed by a newline.
case (FOO)
  ...
endcase
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `tab_character`

### Hint

Replace tab characters with spaces.

### Reason

Tabs may cause misalignment depending on editor setup.

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

Tab characters appear as different widths in dependent on editor/viewer setup,
leading to confusion for readers with a different setup.
Spaces are all but essential, but tabs are not, so this rule simply forbids the
use of tabs.

NOTE: `sv-parser`, the basis of svlint and svls requires files to be encoded
in UTF-8.
See `man iconv` for details on how to convert legacy encodings to UTF-8.

See also:
  - **style_indent** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `uppercamelcase_interface`

### Hint

Begin `interface` name with UpperCamelCase.

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
In Haskell, types/typeclasses must start with an uppercase letter, and
functions/variables must start with a lowercase letter.
This rule checks part of a related naming scheme where modules and interfaces
should start with an uppercase letter, and packages should start with an
lowercase letter.

See also:
  - **lowercamelcase_interface** - Mutually exclusive alternative rule.
  - **lowercamelcase_module** - Potential companion rule.
  - **lowercamelcase_package** - Suggested companion rule.
  - **prefix_interface** - Alternative rule.
  - **uppercamelcase_module** - Suggested companion rule.
  - **uppercamelcase_package** - Potential companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `uppercamelcase_module`

### Hint

Begin `module` name with UpperCamelCase.

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
In Haskell, types/typeclasses must start with an uppercase letter, and
functions/variables must start with a lowercase letter.
This rule checks part of a related naming scheme where modules and interfaces
should start with an uppercase letter, and packages should start with an
lowercase letter.

See also:
  - **lowercamelcase_interface** - Potential companion rule.
  - **lowercamelcase_module** - Mutually exclusive alternative rule.
  - **lowercamelcase_package** - Suggested companion rule.
  - **prefix_module** - Alternative rule.
  - **uppercamelcase_interface** - Suggested companion rule.
  - **uppercamelcase_package** - Potential companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.


---
## `uppercamelcase_package`

### Hint

Begin `package` name with UpperCamelCase.

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

There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
In Haskell, types/typeclasses must start with an uppercase letter, and
functions/variables must start with a lowercase letter.
This rule checks part of a related naming scheme where modules and interfaces
should start with a lowercase letter, and packages should start with an
uppercase letter.

See also:
  - **lowercamelcase_interface** - Suggested companion rule.
  - **lowercamelcase_module** - Suggested companion rule.
  - **lowercamelcase_package** - Mutually exclusive alternative rule.
  - **prefix_package** - Alternative rule.
  - **uppercamelcase_interface** - Potential companion rule.
  - **uppercamelcase_module** - Potential companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.



