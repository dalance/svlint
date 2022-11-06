# Rules

## blocking_assignment_in_always_ff

### Description

blocking assignment is forbidden in `always_ff`

### Reason

blocking assignment in `always_ff` causes elaboration error

### Pass example

```SystemVerilog
module M;
always_ff @(posedge clk) q1 <= d; // Correct.

/* svlint off blocking_assignment_in_always_ff */
always_ff @(posedge clk) q2 = d;  // Control comments avoid failure.
/* svlint on blocking_assignment_in_always_ff */
endmodule
```

### Fail example

```SystemVerilog
module M;
/* svlint off blocking_assignment_in_always_ff */
always_ff @(posedge clk) q1 = d;   // Control comments avoid failure.
/* svlint on blocking_assignment_in_always_ff */

always_ff @(posedge clk) q2 = d;   // Failure.
endmodule
```

## case_default

### Description

`case` must have `default` in `always_comb` or `function`

### Reason

'not full case' causes mismatch between simulation and synthesis

### Pass example

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

### Fail example

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

## default_nettype_none

### Description

`` `default_nettype none`` should be at the top of source code

### Reason

`` `default_nettype none`` can detect unintentional implicit wires

### Pass example

```SystemVerilog
`default_nettype none
module A;
endmodule

```

### Fail example

```SystemVerilog
module A;
endmodule

```

## enum_with_type

### Description

`enum` must have data type

### Reason

the default data type is `int`

### Pass example

```SystemVerilog
module A;
typedef enum logic {
    C
} B;
endmodule
```

### Fail example

```SystemVerilog
module A;
typedef enum {
    C
} B;
endmodule
```

## explicit_case_default

### Description

`case` must have `default` in `always*`

### Reason

explicit `default` makes design intent clearer

### Pass example

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

### Fail example

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

## explicit_if_else

### Description

`if` must have `else` in `always*`

### Reason

explicit `else` makes design intent clearer

### Pass example

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

### Fail example

```SystemVerilog
module A;
always_ff if (x) y <= 0;
always_comb if (x) y = 0;
endmodule
```

## for_with_begin

### Description

multiline `for` statement must have `begin`

### Reason

if there is not `begin`, the second statement are confusing

### Pass example

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

### Fail example

```SystemVerilog
module A;
always_comb begin
    for (int a=0; a<10; a++)
        a = 0;
    for (int a=0; a<10; a++) a = 0;
end
endmodule
```

## function_same_as_system_function

### Description

the name of `function` must not be the same as system function

### Reason

some tools confuse function with system function

### Pass example

```SystemVerilog
module A;
function my_clog2;
endfunction
endmodule
```

### Fail example

```SystemVerilog
module A;
function clog2;
endfunction
endmodule
```

## function_with_automatic

### Description

`function` must be `automatic`

### Reason

this causes mismatch between simulation and synthesis

### Pass example

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

### Fail example

```SystemVerilog
module A;
function A;
endfunction
endmodule
```

## generate_case_with_label

### Description

`generate case item` must have label with prefix "l_"

### Reason

the hierarchiral path can't be determined

### Pass example

```SystemVerilog
module A;
generate case (2'd3)
  2'd1:     begin: l_nondefault wire c = 1'b0; end
  default:  begin: l_default    wire c = 1'b0; end
endcase endgenerate
endmodule
```

### Fail example

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

## generate_for_with_label

### Description

`generate for` must have label with prefix "l_"

### Reason

the hierarchiral path can't be determined

### Pass example

```SystemVerilog
module A;
for(genvar i=0; i<10; i++) begin: l_a
end
endmodule
```

### Fail example

```SystemVerilog
module A;
for(genvar i=0; i<10; i++) foo[i] = i;// noBegin
for(genvar i=0; i<10; i++) begin // noLabel
end
endmodule
```

## generate_if_with_label

### Description

`generate if` must have label with prefix "l_"

### Reason

the hierarchiral path can't be determined

### Pass example

```SystemVerilog
module A;
if (a) begin: l_abc
end else if (b) begin: l_def
end else begin: l_hij
end
endmodule
```

### Fail example

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

## generate_keyword_forbidden

### Description

`generate`/`endgenerate` must be omitted

### Reason



### Pass example

```SystemVerilog
module A;
endmodule
```

### Fail example

```SystemVerilog
module A;
generate
endgenerate
endmodule
```

## generate_keyword_required

### Description

`generate`/`endgenerate` is required

### Reason

some tools don't support `generate`/`endgenerate` omitting

### Pass example

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

### Fail example

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

## genvar_declaration_in_loop

### Description

`genvar` must be declared in loop

### Reason

the scope of variable should be minimized

### Pass example

```SystemVerilog
module A;
for(genvar i=0;i<10;i++) begin: a
end
endmodule
```

### Fail example

```SystemVerilog
module A;
genvar i;
for(i=0;i<10;i++) begin
end
endmodule
```

## genvar_declaration_out_loop

### Description

`genvar` must be declared out loop

### Reason

some tools don't support `genvar` declaration in loop

### Pass example

```SystemVerilog
module A;
genvar i;
for(i=0;i<10;i++) begin: a
end
endmodule
```

### Fail example

```SystemVerilog
module A;
for(genvar i=0;i<10;i++) begin
end
endmodule
```

## if_with_begin

### Description

multiline `if` statement must have `begin`

### Reason

if there is not `begin`, the second statement are confusing

### Pass example

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

### Fail example

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

## inout_with_tri

### Description

`inout` must have `tri`

### Reason



### Pass example

```SystemVerilog
module A (
    inout tri a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    inout wire a
);
endmodule
```

## input_with_var

### Description

`input` must have `var`

### Reason

`input wire` can be assigned by mistake. `input logic` becomes error with `default nettype none` because it doesn't have net type.

### Pass example

```SystemVerilog
module A (
    input var a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    input logic a
);
endmodule
```

## interface_port_with_modport

### Description

interface port must have modport

### Reason

interface port without modport maybe `inout` at synthesis

### Pass example

```SystemVerilog
module A (
    test_if.a a,
    interface.b b
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    test_if a,
    interface b
);
endmodule
```

## legacy_always

### Description

`always_comb`/`always_ff`/`always_latch` must be used

### Reason

`always` can't detect blocking/non-blocking mistake

### Pass example

```SystemVerilog
module A;
always_comb begin
end
endmodule
```

### Fail example

```SystemVerilog
module A;
always @* begin
end
endmodule
```

## level_sensitive_always

### Description

level sensitive `always` must be `always_comb`

### Reason

`always` can't detect blocking/non-blocking mistake

### Pass example

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

### Fail example

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

## localparam_explicit_type

### Description

`localparam` must be have an explicit type

### Reason

parameter types show intent and improve readability

### Pass example

```SystemVerilog
module A;
localparam int a = 0;
endmodule
```

### Fail example

```SystemVerilog
module A;
localparam a = 0;
endmodule
```

## localparam_type_twostate

### Description

`localparam` must be have a twostate type

### Reason

design constants should not contain X or Z bits.

### Pass example

```SystemVerilog
module A;
  localparam byte     a = 0; // 8b
  localparam shortint b = 0; // 16b
  localparam int      c = 0; // 32b
  localparam longint  d = 0; // 64b
  localparam bit      e = 0; // 1b
endmodule
```

### Fail example

```SystemVerilog
module A;
  localparam integer a = 0; // 32b
  localparam logic   b = 0; // 1b
  localparam reg     c = 0; // 1b
endmodule
```

## loop_variable_declaration

### Description

loop variable must be declared in loop

### Reason

the scope of variable should be minimized

### Pass example

```SystemVerilog
module A;
initial begin
for(int i=0;i<10;i++) begin
end
end
endmodule
```

### Fail example

```SystemVerilog
module A;
initial begin
int i;
for(i=0;i<10;i++) begin
end
end
endmodule
```

## lowercamelcase_interface

### Description

Interface name must begin with lowerCamelCase

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
interface fooBar; endinterface
```

### Fail example

```SystemVerilog
interface FooBar; endinterface
```

## lowercamelcase_module

### Description

Module name must begin with lowerCamelCase

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module fooBar; endmodule
```

### Fail example

```SystemVerilog
module FooBar; endmodule
```

## lowercamelcase_package

### Description

Package name must begin with lowerCamelCase

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
package fooBar; endpackage
```

### Fail example

```SystemVerilog
package FooBar; endpackage
```

## non_ansi_module

### Description

module declaration must be ANSI-style

### Reason

non-ANSI-style has duplicated port declaration

### Pass example

```SystemVerilog
module Mansi
  ( input  a
  , output b
  );
endmodule

module Mansi_noPort;
endmodule

module Mansi_defaultInout
  ( a
  , b
  );
endmodule
```

### Fail example

```SystemVerilog
module Mnonansi
  ( a
  , b
  );
  input  a;
  output b;
endmodule
```

## non_blocking_assignment_in_always_comb

### Description

non-blocking assignment is forbidden in`always_comb`

### Reason

non-blocking assignment in `always_comb` causes elaboration error

### Pass example

```SystemVerilog
module A;
always_comb begin
    x = 0;
end
endmodule
```

### Fail example

```SystemVerilog
module A;
always_comb begin
    x <= 0;
end
endmodule
```

## output_with_var

### Description

`output` must have `var`

### Reason



### Pass example

```SystemVerilog
module A (
    output var a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    output logic a
);
endmodule
```

## parameter_explicit_type

### Description

`parameter` must be have an explicit type

### Reason

parameter types show intent and improve readability

### Pass example

```SystemVerilog
module A #(parameter int a = 0) ();
endmodule
```

### Fail example

```SystemVerilog
module A #(parameter a = 0) ();
endmodule
```

## parameter_in_package

### Description

`parameter` must be replaced to `localparam` in `package`

### Reason

some tools can't take `parameter` in `package`

### Pass example

```SystemVerilog
package A;
localparam A = 1;
endpackage
```

### Fail example

```SystemVerilog
package A;
parameter A = 1;
endpackage
```

## parameter_type_twostate

### Description

`parameter` must be have a twostate type

### Reason

design constants should not contain X or Z bits.

### Pass example

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

### Fail example

```SystemVerilog
module A #(
  parameter integer a = 0, // 32b
  parameter logic   b = 0, // 1b
  parameter reg     c = 0  // 1b
) ();
endmodule
```

## prefix_inout

### Description

`inout` must have prefix "b_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module M
( inout var b_foo
, input var logic [FOO-1:0] b_bar
);
endmodule
```

### Fail example

```SystemVerilog
module M
( inout var foo
, inout var logic [FOO-1:0] bar
);
endmodule
```

## prefix_input

### Description

`input` must have prefix "i_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module M
( input var i_foo
, input var logic [FOO-1:0] i_bar
);
endmodule
```

### Fail example

```SystemVerilog
module M
( input var foo
, input var logic [FOO-1:0] bar
);
endmodule
```

## prefix_instance

### Description

Module instance must have prefix "u_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module A;
Foo #() u_foo (a, b, c);
endmodule
```

### Fail example

```SystemVerilog
module A;
Foo #() foo (a, b, c);
endmodule
```

## prefix_interface

### Description

`interface` name must have prefix "ifc_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
interface ifc_withPrefix; endinterface
```

### Fail example

```SystemVerilog
interface noPrefix; endinterface
```

## prefix_module

### Description

`module` name must have prefix "mod_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module mod_withPrefix; // Module identifier of declaration has prefix.
  M #(.A(1)) u_M (.a); // Module identifier of instance doesn't require prefix.
endmodule
```

### Fail example

```SystemVerilog
module noPrefix; // Module identifier of declaration should have prefix.
endmodule
```

## prefix_output

### Description

`output` must have prefix "o_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module M
( output var o_foo
, output var logic [FOO-1:0] o_bar
);
endmodule
```

### Fail example

```SystemVerilog
module M
( output var foo
, output var logic [FOO-1:0] bar
);
endmodule
```

## prefix_package

### Description

`package` name must have prefix "pkg_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
package pkg_withPrefix; endpackage
```

### Fail example

```SystemVerilog
package noPrefix; endpackage
```

## priority_keyword

### Description

`priority` is forbidden

### Reason

this causes mismatch between simulation and synthesis

### Pass example

```SystemVerilog
module A();
initial begin
    case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Fail example

```SystemVerilog
module A();
initial begin
    priority case (a)
        default: b = 1;
    endcase
end
endmodule
```

## re_forbidden_checker

### Description

Use a checker identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
checker Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endchecker
```

### Fail example

```SystemVerilog
checker foo; // Unconfigured forbidden regex matches (almost) anything.
endchecker
```

## re_forbidden_class

### Description

Use a class identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
class Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endclass
```

### Fail example

```SystemVerilog
class foo; // Unconfigured forbidden regex matches (almost) anything.
endclass
```

## re_forbidden_function

### Description

Use a function identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
package P;
  function Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
  endfunction
endpackage
```

### Fail example

```SystemVerilog
package P;
  function foo; // Unconfigured forbidden regex matches (almost) anything.
  endfunction
endpackage
```

## re_forbidden_interface

### Description

Use a interface identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
interface Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endinterface
```

### Fail example

```SystemVerilog
interface foo; // Unconfigured forbidden regex matches (almost) anything.
endinterface
```

## re_forbidden_localparam

### Description

Use a localparam identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
package P;
  localparam Xfoo = 0; // Identifier doesn't match default forbidden regex (X prefix).
endpackage
```

### Fail example

```SystemVerilog
package P;
  localparam foo = 0; // Unconfigured forbidden regex matches (almost) anything.
endpackage
```

## re_forbidden_modport

### Description

Use a modport identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
interface I;
  modport Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  ( input i
  );
endinterface
```

### Fail example

```SystemVerilog
interface I;
  modport foo // Unconfigured forbidden regex matches (almost) anything.
  ( input i
  );
endinterface
```

## re_forbidden_module_ansi

### Description

Use a module identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
```

### Fail example

```SystemVerilog
module foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
```

## re_forbidden_module_nonansi

### Description

Use a module identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  ( a
  );
  input a;
endmodule
```

### Fail example

```SystemVerilog
module foo // Unconfigured forbidden regex matches (almost) anything.
  ( a
  );
  input a;
endmodule
```

## re_forbidden_package

### Description

Use a package identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
package Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endpackage
```

### Fail example

```SystemVerilog
package foo; // Unconfigured forbidden regex matches (almost) anything.
endpackage
```

## re_forbidden_parameter

### Description

Use a parameter identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  #( Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  ) ();
endmodule
```

### Fail example

```SystemVerilog
module M
  #( foo // Unconfigured forbidden regex matches (almost) anything.
  ) ();
endmodule
```

## re_forbidden_port_inout

### Description

Use a port identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( inout Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  inout Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( inout foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  inout foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
```

## re_forbidden_port_input

### Description

Use a port identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( input Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  input Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( input foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  input foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
```

## re_forbidden_port_interface

### Description

Use a port identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( I Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  I.i Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( I.i foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  I.i foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
```

## re_forbidden_port_output

### Description

Use a port identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( output Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  output Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( output foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  output foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
```

## re_forbidden_port_ref

### Description

Use a port identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( ref Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  ref var Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( ref foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  ref var foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
```

## re_forbidden_program

### Description

Use a program identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
program Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endprogram
```

### Fail example

```SystemVerilog
program foo; // Unconfigured forbidden regex matches (almost) anything.
endprogram
```

## re_forbidden_task

### Description

Use a task identifier not matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M;
  task Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
  endtask
endmodule
```

### Fail example

```SystemVerilog
module M;
  task foo; // Unconfigured forbidden regex matches (almost) anything.
  endtask
endmodule
```

## re_forbidden_var_class

### Description

Use a class-scoped variable identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
class C;
  int Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endclass
```

### Fail example

```SystemVerilog
class C;
  int foo; // Unconfigured forbidden regex matches (almost) anything.
endclass
```

## re_forbidden_var_classmethod

### Description

Use a method-scoped variable identifier matching regex "^[^X](UNCONFIGURED|.*)$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
class C;
  function F;
    int Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
  endfunction
endclass
```

### Fail example

```SystemVerilog
class C;
  function F;
    int foo; // Unconfigured forbidden regex matches (almost) anything.
  endfunction
endclass
```

## re_required_checker

### Description

Use a checker identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
checker mn3; // Identifier matches default required regex (lowercase).
endchecker
```

### Fail example

```SystemVerilog
checker Mn3; // Identifier doesn't match default required regex (lowercase).
endchecker
```

## re_required_class

### Description

Use a class identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
class mn3; // Identifier matches default required regex (lowercase).
endclass
```

### Fail example

```SystemVerilog
class Mn3; // Identifier doesn't match default required regex (lowercase).
endclass
```

## re_required_function

### Description

Use a function identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
package P;
  function mn3; // Identifier matches default required regex (lowercase).
  endfunction
endpackage
```

### Fail example

```SystemVerilog
package P;
  function Mn3; // Identifier doesn't match default required regex (lowercase).
  endfunction
endpackage
```

## re_required_interface

### Description

Use a interface identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
interface mn3; // Identifier matches default required regex (lowercase).
endinterface
```

### Fail example

```SystemVerilog
interface Mn3; // Identifier doesn't match default required regex (lowercase).
endinterface
```

## re_required_localparam

### Description

Use a localparam identifier matching regex "^[A-Z]+[A-Z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
package P;
  localparam MN3 = 0; // Identifier matches default required regex (uppercase).
endpackage
```

### Fail example

```SystemVerilog
package P;
  localparam Mn3 = 0; // Identifier doesn't match default required regex (uppercase).
endpackage
```

## re_required_modport

### Description

Use a modport identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
interface I;
  modport mn3 // Identifier matches default required regex (lowercase).
  ( input i
  );
endinterface
```

### Fail example

```SystemVerilog
interface I;
  modport Mn3 // Identifier doesn't match default required regex (lowercase).
  ( input i
  );
endinterface
```

## re_required_module_ansi

### Description

Use a module identifier matching regex "^[a-z]+[a-zA-Z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module mN3; // Identifier matches default required regex (mixed-case).
endmodule
```

### Fail example

```SystemVerilog
module Mn3; // Identifier doesn't match default required regex (mixed-case).
endmodule
```

## re_required_module_nonansi

### Description

Use a module identifier matching regex "^[A-Z]+[A-Z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module MN3 // Identifier matches default required regex (uppercase).
  ( a
  );
  input a;
endmodule
```

### Fail example

```SystemVerilog
module mn3 // Identifier doesn't match default required regex (uppercase).
  ( a
  );
  input a;
endmodule
```

## re_required_package

### Description

Use a package identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
package mn3; // Identifier matches default required regex (lowercase).
endpackage
```

### Fail example

```SystemVerilog
package Mn3; // Identifier doesn't match default required regex (lowercase).
endpackage
```

## re_required_parameter

### Description

Use a parameter identifier matching regex "^[A-Z]+[A-Z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  #( MN3 // Identifier matches default required regex (uppercase).
  ) ();
endmodule
```

### Fail example

```SystemVerilog
module M
  #( Mn3 // Identifier doesn't match default required regex (uppercase).
  ) ();
endmodule
```

## re_required_port_inout

### Description

Use a port identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( inout mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  inout mn3; // Identifier matches default required regex (lowercase).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( inout Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  inout Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
```

## re_required_port_input

### Description

Use a port identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( input mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  input mn3; // Identifier matches default required regex (lowercase).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( input Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  input Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
```

## re_required_port_interface

### Description

Use a port identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( I.i mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  I.i mn3; // Identifier matches default required regex (lowercase).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( I.i Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  I.i Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
```

## re_required_port_output

### Description

Use a port identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( output mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  output mn3; // Identifier matches default required regex (lowercase).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( output Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  output Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
```

## re_required_port_ref

### Description

Use a port identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M
  ( ref mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  ref var mn3; // Identifier matches default required regex (lowercase).
endmodule
```

### Fail example

```SystemVerilog
module M
  ( ref Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  ref var Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
```

## re_required_program

### Description

Use a program identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
program mn3; // Identifier matches default required regex (lowercase).
endprogram
```

### Fail example

```SystemVerilog
program Mn3; // Identifier doesn't match default required regex (lowercase).
endprogram
```

## re_required_task

### Description

Use a task identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
module M;
  task mn3; // Identifier matches default required regex (lowercase).
  endtask
endmodule
```

### Fail example

```SystemVerilog
module M;
  task Mn3; // Identifier doesn't match default required regex (lowercase).
  endtask
endmodule
```

## re_required_var_class

### Description

Use a class-scoped variable identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
class C;
  int mn3; // Identifier matches default required regex (lowercase).
endclass
```

### Fail example

```SystemVerilog
class C;
  int Mn3; // Identifier doesn't match default required regex (lowercase).
endclass
```

## re_required_var_classmethod

### Description

Use a method-scoped variable identifier matching regex "^[a-z]+[a-z0-9_]*$".

### Reason

Identifiers must conform to the naming scheme.

### Pass example

```SystemVerilog
class C;
  function F;
    int mn3; // Identifier matches default required regex (lowercase).
  endfunction
endclass
```

### Fail example

```SystemVerilog
class C;
  function F;
    int Mn3; // Identifier doesn't match default required regex (lowercase).
  endfunction
endclass
```

## sequential_block_in_always_comb

### Description

begin/end forbidden within `always_comb` constuct

### Reason

prevent introducing sequential dependencies

### Pass example

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

### Fail example

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

## sequential_block_in_always_ff

### Description

begin/end forbidden within `always_ff` constuct

### Reason

prevent introducing sequential dependencies

### Pass example

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

### Fail example

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

## sequential_block_in_always_latch

### Description

begin/end forbidden within `always_latch` constuct

### Reason

prevent introducing sequential dependencies

### Pass example

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

### Fail example

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

## style_commaleading

### Description

comma should be followed by a single space (comma-leading format)

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

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

## style_indent

### Description

newline should be followed by a multiple of 2 spaces

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

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

## style_keyword_0or1space

### Description

keyword should be followed by a symbol or exactly 1 space

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

```SystemVerilog
module A;
  function foo();
    if (a)
      return  ; // multiple spaces after `return`.
  endfunction
endmodule

```

## style_keyword_0space

### Description

keyword should be followed by no space before symbol

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

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

## style_keyword_1or2space

### Description

keyword should be followed by exactly 1 or 2 spaces

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

```SystemVerilog
module M (
  input   a,
  inout   b   // multiple spaces after `input` or `inout` keywords
);
endmodule
```

## style_keyword_1space

### Description

keyword should be followed by a single space

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

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

## style_keyword_construct

### Description

keyword should be followed by newline or exactly 1 space

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

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

## style_keyword_datatype

### Description

keyword should be followed by a single space

### Reason

consistent style enhances readability

### Pass example

```SystemVerilog
module M;
  localparam bit A = 0;   // 1 space after `bit`.
  localparam int B = 0;   // 1 space after `int`.
  logic a;                // 1 space after `logic`.
  reg b;                  // 1 space after `reg`.
  wire b;                 // 1 space after `wire`.
endmodule
```

### Fail example

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

## style_keyword_end

### Description

keyword should be followed by newline, colon, or exactly 1 space

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

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

## style_keyword_maybelabel

### Description

keyword should be followed by newline or colon, not spaces

### Reason

consistent style enhances readability

### Pass example

```SystemVerilog
module A;
endmodule: A // colon immediately after `endmodule`
package A;
    function foo();
    endfunction
//  ^^^^^^^^^^^ newline after `endfunction`
endpackage // 1 space then comment after `endpackage`

```

### Fail example

```SystemVerilog
module A;
endmodule  : A // spaces immediately after `endmodule`
package A;
endpackage  // multiple spaces then comment after `endpackage`
interface A;
endinterface interface B; // space instead of newline after `endinterface`
endinterface
```

## style_keyword_newline

### Description

keyword should be followed by a newline

### Reason

consistent style enhances readability

### Pass example

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

### Fail example

```SystemVerilog
module A;
  generate
    case (foo)
      123: a = b;
    endcase if (foo) a = b; // no newline after `endcase`
  endgenerate   // multiple spaces then comment after `endgenerate`
endmodule

```

## tab_character

### Description

tab character is forbidden

### Reason

may cause misalignment depending on editor setting

### Pass example

```SystemVerilog
module A();
  logic a;
endmodule
```

### Fail example

```SystemVerilog
module A();
	logic a;
endmodule
```

## unique0_keyword

### Description

`unique0` is forbidden

### Reason

this causes mismatch between simulation and synthesis

### Pass example

```SystemVerilog
module A();
initial begin
    case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Fail example

```SystemVerilog
module A();
initial begin
    unique0 case (a)
        default: b = 1;
    endcase
end
endmodule
```

## unique_keyword

### Description

`unique` is forbidden

### Reason

this causes mismatch between simulation and synthesis

### Pass example

```SystemVerilog
module A();
initial begin
    case (a)
        default: b = 1;
    endcase
end
endmodule
```

### Fail example

```SystemVerilog
module A();
initial begin
    unique case (a)
        default: b = 1;
    endcase
end
endmodule
```

## uppercamelcase_interface

### Description

Interface name must begin with UpperCamelCase

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
interface FooBar; endinterface
```

### Fail example

```SystemVerilog
interface fooBar; endinterface
```

## uppercamelcase_module

### Description

Module name must begin with UpperCamelCase

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module FooBar; endmodule
```

### Fail example

```SystemVerilog
module fooBar; endmodule
```

## uppercamelcase_package

### Description

Package name must begin with UpperCamelCase

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
package FooBar; endpackage
```

### Fail example

```SystemVerilog
package fooBar; endpackage
```

## wire_reg

### Description

`wire`/`reg` must be replaced to `logic`/`tri`

### Reason

`logic` can detect multi-drive

### Pass example

```SystemVerilog
module A;
logic a;
logic b;
endmodule
```

### Fail example

```SystemVerilog
module A;
wire a;
reg b;
endmodule
```

