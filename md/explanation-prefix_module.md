There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
This rule requires that module identifiers are declared with a prefix of `mod_`
(configurable) which allows a reader to easily distinguish between
module and interface instances.

See also:

- **lowercamelcase_module** - Alternative rule.
- **prefix_interface** - Suggested companion rule.
- **prefix_package** - Suggested companion rule.
- **uppercamelcase_module** - Alternative rule.
