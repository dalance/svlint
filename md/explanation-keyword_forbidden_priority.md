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
