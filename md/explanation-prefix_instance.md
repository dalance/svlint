This rule requires that instances of modules or interfaces are prefixed with
`u_` (configurable) which allows readers to quickly find instances and
connections of interest.
Prefixing instances also allows components of a hierarchical path to be easily
identified as modules/interfaces rather than generate blocks, which is
especially useful when reading netlists and synthesis reports.
The default value of `u_` comes from the historical use of `U` for the PCB
reference designator of an inseparable assembly or integrated-circuit package,
as standardized in IEEE315-1975.

See also:

- **generate_case_with_label** - Suggested companion rule.
- **generate_for_with_label** - Suggested companion rule.
- **generate_if_with_label** - Suggested companion rule.
- **prefix_inout** - Suggested companion rule.
- **prefix_input** - Suggested companion rule.
- **prefix_output** - Suggested companion rule.
- <https://en.wikipedia.org/wiki/Reference_designator>
