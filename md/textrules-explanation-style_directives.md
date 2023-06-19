Check that (most) preprocessor and compiler directives are not indented, and
that there are no items preceeding a directive on the same line.

There are 22 compiler directives defined in IEEE1800-2017:

- `begin_keywords`
- `end_keywords`
- `celldefine`
- `endcelldefine`
- `unconnected_drive`
- `nounconnected_drive`
- `pragma`
- `timescale`
- `default_nettype`
- `line`
- `resetall`
- `__LINE__`
- `__FILE__`
- `include`
- `define`
- `ifdef`
- `ifndef`
- `elsif`
- `else`
- `endif`
- `undef`
- `undefineall`

Each of these can have profound effects on the surrounding source code, so it's
important that these stand out such that they're difficult to overlook.
To ensure that directives are prominently displayed, and to discourage
deep/complex ifdef logic, this rule uses a regular expression to check that
there are no characters before any directive (excluding `__LINE__` or
`__FILE__`).
This does not affect user-defined preprocessor macros.

See also:
- The "Indentation Preprocessor Considerations" section of **ruleset-style**.

The most relevant clauses of IEEE1800-2017 are:
- 22 Compiler directives
