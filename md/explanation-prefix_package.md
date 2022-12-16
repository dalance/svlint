There are 3 usual types of SystemVerilog file for synthesizable design code
(module, interface, package) and having a simple naming convention helps
distinguish them from a filesystem viewpoint.
This rule requires that package identifiers are declared with a prefix of
`pkg_` (configurable).
When used in conjunction with a file naming scheme like "There should be one
package declaration per file, and a package `pkg_foo` must be contained in a
file called `pkg_foo.sv`.", this aids a reader in browsing a source directory.

See also:
  - **lowercamelcase_package** - Alternative rule.
  - **prefix_interface** - Suggested companion rule.
  - **prefix_module** - Potential companion rule.
  - **uppercamelcase_package** - Alternative rule.
