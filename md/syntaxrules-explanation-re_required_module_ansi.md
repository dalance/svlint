Modules declared with an ANSI header must have identifiers matching the regex
configured via the `re_required_module_ansi` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
- **re_forbidden_module_ansi**
- **re_forbidden_module_nonansi**
- **re_required_module_nonansi**
- **prefix_module**
- **uppercamelcase_module**
- **lowercamelcase_module**
- **module_nonansi_forbidden**
