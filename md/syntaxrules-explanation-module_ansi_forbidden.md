There are two ways to declare a module header in SystemVerilog:
1. ANSI style - newer, neater, more succinct, mostly compatible with
  IEEE1364-2001 (as long as you don't use `localparam`s for ports).
2. non-ANSI style - additionally compatible with older Verilog (IEEE1364-1995).

Examples of both styles are given in IEEE1364-2001 (e.g. pages 180 vs 182) and
IEEE1800-2017 (e.g. pages 702 vs 700).

The non-ANSI style separates the declaration of ports, their direction, and
their datatype.
While requiring more text, and visual noise, to convey the same information,
the non-ANSI style allows non-overridable parameters, i.e. `localparam`, to be
used in port declarations.
If only `parameter` is used instead, as allowed in IEEE1364, an instance may
inadvertently override a parameter, thus causing difficult-to-debug issues.

This rule requires that module headers are declared using the non-ANSI style.
It is recommended only to use this rule where compatibility with IEEE1364 is
required.
By forbidding the ANSI style, this rule requires that module declarations are
written in a consistent manner, which facilitates easier review and prevents
easily overlooked issues before they become problems.

See also:
- **module_nonansi_forbidden** - For safer usability where compatibility with
  Verilog is not required.

The most relevant clauses of IEEE1364-2001 are:
- 12.1 Modules
- 12.2 Overriding module parameter values

The most relevant clauses of IEEE1800-2017 are:
- 23.2 Module definitions
