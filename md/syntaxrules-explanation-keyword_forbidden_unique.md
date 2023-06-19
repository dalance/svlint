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
