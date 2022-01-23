# Rules

## blocking_assignment_in_always_ff

### Description

blocking assignment is forbidden in `always_ff`

### Reason

blocking assignment in `always_ff` causes elaboration error

### Pass example

```SystemVerilog
module A;
always_ff begin
    x <= 0;
end
endmodule
```

### Fail example

```SystemVerilog
module A;
always_ff begin
    x = 0;
end
endmodule
```

## case_default

### Description

`case` must have `default` in `always_comb`, `always_ff` or `function`

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
module A(
    input  a,
    output b
);
endmodule
```

### Fail example

```SystemVerilog
module A(
    a,
    b
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

## prefix_inout

### Description

`inout` must have prefix "b_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module A (
    inout var b_a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    inout var a
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
module A (
    input var i_a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    input var a
);
endmodule
```

## prefix_input

### Description

Module instance must have prefix "u_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module A (
    input var i_a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    input var a
);
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
module mod_withPrefix; endmodule
```

### Fail example

```SystemVerilog
module noPrefix; endmodule
```

## prefix_output

### Description

`output` must have prefix "o_"

### Reason

Naming convention simplifies audit.

### Pass example

```SystemVerilog
module A (
    output var o_a
);
endmodule
```

### Fail example

```SystemVerilog
module A (
    output var a
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

