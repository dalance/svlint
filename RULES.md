# Rules

This document is generated from the rules' source code (`svlint/src/rules/*.rs`)
and testcases (`testcases/(fail|pass)/*.sv`) using the `mdgen` utility.
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

`` `default_nettype none`` should be at the top of source code

### Reason

`` `default_nettype none`` can detect unintentional implicit wires

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

The `\`default_netype` compiler directive can be used to specify the net type
of implicit nets, i.e. where a signal is referenced, or assigned to, without
being declared.
IEEE1800-2017 clause 22.8 stipulates "When no `\`default_nettype` directive
is present or if the `\`resetall` directive is specified, implicit nets are of
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
When `\`default_nettype none` is used, all signals must be declared, thus
forcing the author to consider whether they mean a variable or a net.

The most relevant clauses of IEEE1800-2017 are:
  - 6.5 Nets and variables
  - 22.8 default nettype

Note: One prominent paper (Cliff Cummings, HDLCON 2002) recommends *against*
using `\`default_nettype none` on the basis that concise, typeless code has
fewer opportunities for mistakes.
This attitude was popular at the time, e.g. Python's dynamic typing, but
modern attitudes are now favouring explicit types, e.g. Python's new type
checking syntax and tooling.
Additionally, the reasoning behind this guideline only applies principally to
IEEE1364, but not strongly to IEEE1800.


---
## `enum_with_type`

### Hint

`enum` must have data type

### Reason

the default data type is `int`

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

TODO

---
## `explicit_case_default`

### Hint

`case` must have `default` in `always*`

### Reason

explicit `default` makes design intent clearer

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

TODO

---
## `explicit_if_else`

### Hint

`if` must have `else` in `always*`

### Reason

explicit `else` makes design intent clearer

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

TODO

---
## `for_with_begin`

### Hint

multiline `for` statement must have `begin`

### Reason

if there is not `begin`, the second statement are confusing

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

TODO

---
## `function_same_as_system_function`

### Hint

the name of `function` must not be the same as system function

### Reason

some tools confuse function with system function

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

TODO

---
## `function_with_automatic`

### Hint

`function` must be `automatic`

### Reason

this causes mismatch between simulation and synthesis

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

TODO

---
## `generate_case_with_label`

### Hint

`generate case item` must have label with prefix "l_"

### Reason

the hierarchiral path can't be determined

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

TODO

---
## `generate_for_with_label`

### Hint

`generate for` must have label with prefix "l_"

### Reason

the hierarchiral path can't be determined

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

TODO

---
## `generate_if_with_label`

### Hint

`generate if` must have label with prefix "l_"

### Reason

the hierarchiral path can't be determined

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

TODO

---
## `generate_keyword_forbidden`

### Hint

`generate`/`endgenerate` must be omitted

### Reason



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

TODO

---
## `generate_keyword_required`

### Hint

`generate`/`endgenerate` is required

### Reason

some tools don't support `generate`/`endgenerate` omitting

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

TODO

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
## `if_with_begin`

### Hint

multiline `if` statement must have `begin`

### Reason

if there is not `begin`, the second statement are confusing

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

`localparam` must be have a twostate type

### Reason

design constants should not contain X or Z bits.

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

TODO

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

begin/end forbidden within `always_comb` constuct

### Reason

prevent introducing sequential dependencies

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

TODO

---
## `sequential_block_in_always_ff`

### Hint

begin/end forbidden within `always_ff` constuct

### Reason

prevent introducing sequential dependencies

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

TODO

---
## `sequential_block_in_always_latch`

### Hint

begin/end forbidden within `always_latch` constuct

### Reason

prevent introducing sequential dependencies

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

