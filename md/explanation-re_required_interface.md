Interfaces must have identifiers matching the regex configured via the
`re_required_interface` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
- **re_forbidden_interface**
- **prefix_interface**
- **uppercamelcase_interface**
- **lowercamelcase_interface**
