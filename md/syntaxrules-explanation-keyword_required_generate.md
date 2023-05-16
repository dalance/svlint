The `generate`/`endgenerate` keywords may be used in a module, interface,
program, or checker to define a generate region.
A generate region is a textual span in the module description where generate
constructs may appear.
Use of generate regions is optional.
There is no semantic difference in the module when a generate region is used.
A parser may choose to recognize the generate region to produce different error
messages for misused generate construct keywords.

Some non-compliant tools may require the use of these keywords.
Therefore, this rule is designed to mandate their use.

NOTE: The visual noise introduced by these keywords provides an argument
against this rule.

See also:
- **keyword_forbidden_generate** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
- 27.3 Generate construct syntax
