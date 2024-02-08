This rule is an extension of the **case_default** rule that allows the case default to be implicitly defined.
Case statements without a `default` branch can cause signals to be undriven. Setting default values of signals at the top of an `always` procedures is good practice and ensures that signals are never metastable when a case match fails. For example,
```sv
always_comb begin
  y = 0;
  case(x)
    1: y = 1;
  endcase
end

```
If the case match fails, `y` wouldn't infer memory or be undriven because the default value is defined before the `case`.

See also:
 - **case_default**
 - **explicit_case_default**

The most relevant clauses of IEEE1800-2017 are:

- 12.5 Case statement

