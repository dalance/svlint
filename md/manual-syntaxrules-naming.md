# Naming Convention Syntax Rules

Rules for checking against naming conventions are named with either the suffix
`_with_label` or one of these prefixes:

- `prefix_`
- `(lower|upper)camelcase_`
- `re_(forbidden|required)_`

Naming conventions are useful to help ensure consistency across components in
large projects.
An example of naming-convention ruleset is given in
**ruleset-DaveMcEwan-design**.

The rules `re_forbidden_*` can also be used to restrict language features.
For example, if a project requires that interfaces must never be used, you can
enable the rule `re_forbidden_interface` and configure it to match all
identifier strings.
By forbidding all possible identifiers at the point of declaration, no
interfaces may be specified.
For example:
```toml
[option]
re_forbidden_interface = ".*"

[rules]
re_forbidden_interface = true
```
