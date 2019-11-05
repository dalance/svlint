# Rules

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

if there is not `begin`, the second statatement are confusing

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

this causes mismatch between simulaton and synthesis

### Pass example

```SystemVerilog
module A;
function automatic A;
endfunction
endmodule
```

### Fail example

```SystemVerilog
module A;
function A;
endfunction
endmodule
```

## generate_for_with_label

### Description

`generate for` must have label

### Reason

the hierarchiral path can't be determined

### Pass example

```SystemVerilog
module A;
for(genvar i=0; i<10; i++) begin: a
end
endmodule
```

### Fail example

```SystemVerilog
module A;
for(genvar i=0; i<10; i++) begin
end
endmodule
```

## generate_if_with_label

### Description

`generate if` must have label

### Reason

the hierarchiral path can't be determined

### Pass example

```SystemVerilog
module A;
if (a) begin: a
end else if (b) begin: a
end else begin: a
end
endmodule
```

### Fail example

```SystemVerilog
module A;
if (a) begin
end else if (a) begin
end else begin
end
endmodule
```

## generate_keyword

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

## genvar_declaration

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

## if_with_begin

### Description

multiline `if` statement must have `begin`

### Reason

if there is not `begin`, the second statatement are confusing

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

## priority_keyword

### Description

`priority` is forbidden

### Reason

this causes mismatch between simulaton and synthesis

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

## tab_charactor

### Description

tab charactor is forbidden

### Reason

this causes broken indent by editor setting

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

this causes mismatch between simulaton and synthesis

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

this causes mismatch between simulaton and synthesis

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

