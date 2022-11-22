Input ports must not have identifiers matching the regex configured via the
`re_forbidden_port_input` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
  - **re_required_port_input**
  - **prefix_input**
