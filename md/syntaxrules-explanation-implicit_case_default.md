This rule is an extension of the **case_default** rule that allows the case
default to be implicitly defined. Case statements without a `default` branch
can cause signals to be undriven. Setting default values of signals at the top
of an `always` procedures is good practice and ensures that signals are never
metastable when a case match fails. For example,

```sv
always_comb begin
  y = 0;
  case (x)
    1: y = 1;
  endcase
end

```

If the case match on `x` fails, `y` would not infer memory or be undriven
because the default value is defined before the `case`.

This rule is a more lenient version of `case_default`. It adapts to a specific
coding style of setting default values to signals at the top of a procedural
block to ensure that signals have a default value regardless of the logic in the
procedural block. As such, this rule will only consider values set
**unconditionally** at the top of the procedural block as a default and will
disregard assignments made in conditional blocks like `if`/`else`, etc. If this
coding style is not preferred, it is strongly suggested to use the rules
mentioned below as they offer stricter guarantees.

See also:

- **case_default**
- **explicit_case_default**

The most relevant clauses of IEEE1800-2017 are:

- 12.5 Case statement
