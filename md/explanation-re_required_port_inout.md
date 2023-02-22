Bidirectional ports must have identifiers matching the regex configured via the
`re_required_port_inout` option.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
- **re_forbidden_inout**
- **prefix_inout**
