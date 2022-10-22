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

The most relevant clauses of IEEE1800-2017 are:
  - 6.8 Variable declarations
  - 6.11 Integer data types
  - 6.19 Enumerations
  - Table 6.7 Default variable initial values
  - Table 6.8 Integer data types
