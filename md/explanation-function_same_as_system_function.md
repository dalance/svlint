IEEE1800-2017 provides a variety of built-in functions, which must be
implemented in simulation and synthesis tools.
This rule is designed to catch (possibly incorrect) re-implementations of these
functions which may have different behavior and confuse readers.
Additionally, some tools may (wrongly) confuse user-defined functions with the
built-in system of the same name (except of the leading `$`) which may lead
to inconsistent results between tools.

See also:
- **function_with_automatic** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 13.7 Task and function names
- 20 Utility system tasks and system functions
- 23.8.1 Task and function name resolution
