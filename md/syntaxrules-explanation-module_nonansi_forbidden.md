There are two ways to declare a module header in SystemVerilog:
1. ANSI style - newer, neater, more succinct, mostly compatible with
  IEEE1364-2001 (as long as you don't use `localparam`s for ports).
2. non-ANSI style - additionally compatible with older Verilog (IEEE1364-1995).

Examples of both styles are given in IEEE1364-2001 (e.g. pages 180 vs 182) and
IEEE1800-2017 (e.g. pages 702 vs 700).

The non-ANSI style separates the declaration of ports, their direction, and
their datatype.
In addition to requiring more text, and visual noise, to convey the same
information, the non-ANSI style encourages simple coding mistakes where
essential attributes may be forgotten.
This rule requires that module headers are declared using the ANSI style.

See also:
- **module_ansi_forbidden** - For consistency in IEEE1364-2001 (compatibility
  with non-overridable parameters, i.e. `localparam`, in port declarations,
  or compatibility with IEEE1364-1995.

The most relevant clauses of IEEE1364-2001 are:
- 12.1 Modules
- 12.2 Overriding module parameter values

The most relevant clauses of IEEE1800-2017 are:
- 23.2 Module definitions
