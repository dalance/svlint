Interfaces must not have identifiers matching the regex configured via the
`re_forbidden_interface` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:

- **re_required_interface**
- **prefix_interface**
- **uppercamelcase_interface**
- **lowercamelcase_interface**
