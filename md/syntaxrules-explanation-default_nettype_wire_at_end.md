Ensures that the final effective `default_nettype` in each source file is `wire`. This prevents a modified `default_nettype` directive from unintentionally affecting other files within the same compilation unit.

This rule is intended to be used together with rules that enforce a specific `default_nettype` at the beginning of a file (for example, `default_nettype_none`). Using both ensures that each file explicitly sets and then restores the directive.

See also:
* **default_nettype_none** – Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
* 22.8 `default_nettype`
