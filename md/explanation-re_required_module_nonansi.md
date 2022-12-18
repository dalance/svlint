Modules declared with a non-ANSI header must have identifiers matching the
regex configured via the `re_required_module_nonansi` option.
Non-ANSI modules are commonly used where compatability with classic Verilog
(IEEE1364-1995) is required, such as low-level cells and macros.

NOTE: For performance reasons, particularly within text-editor integrations
(i.e. svls), the `re_(required|forbidden)_` should only be used where the
simpler naming rules are not sufficient.

See also:
  - **re_forbidden_module_nonansi**
  - **re_forbidden_module_ansi**
  - **re_required_module_ansi**
  - **prefix_module**
  - **uppercamelcase_module**
  - **lowercamelcase_module**
  - **non_ansi_module**
