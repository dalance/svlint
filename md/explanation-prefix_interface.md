There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
This rule requires that interface identifiers are declared with a prefix of
`ifc_` (configurable) which allows a reader to easily distinguish between
module and interface instances.

See also:
  - **lowercamelcase_interface** - Alternative rule.
  - **prefix_module** - Potential companion rule.
  - **prefix_package** - Suggested companion rule.
  - **uppercamelcase_interface** - Alternative rule.
