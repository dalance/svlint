Generate blocks must have identifiers matching the regex configured via the
`re_required_generateblock` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
  - **re_forbidden_generateblock**
  - **generate_case_with_label**
  - **generate_for_with_label**
  - **generate_if_with_label**

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.
