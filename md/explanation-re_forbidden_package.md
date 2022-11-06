Packages must not have identifiers matching the regex configured via the
`re_forbidden_package` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
  - **re_required_package**
  - **prefix_package**
  - **uppercamelcase_package**
  - **lowercamelcase_package**

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.