In order to avoid subtle bugs related to 2-state vs 4-state types and
X-propagation, constants should be declared with an explicit 2-state type.
Separately, all synthesizable signals should be declared with an explicit
4-state type so that a simulation can detect unknown values (Xs).
For complex types such as nested packed structs, that means you need two
versions of each type: a 2-state version for constants, and a 4-state version
for signals.
The need for this rule stems from the fact that SystemVerilog includes the
concepts of both equivalence and partial equivalence, with interactions between
2-state and 4-state structure members which invite mismatching behavior
between simulation and synthesis.

The relevant quote about implicit conversion of packed structure members from
2-state to 4-state is found on page 140 of IEEE1800-2017:
If all datatypes within a packed structure are 2-state, the structure as a
whole is treated as a 2-state vector.
If any datatype within a packed structure is 4-state, the structure as a whole
is treated as a 4-state vector.
If there are also 2-state members in the structure, there is an implicit
conversion from 4-state to 2-state when reading those members and from 2-state
to 4-state when writing them.

For constants of simple datatypes, it is trivial to visually check that their
values do not contain Xs or Zs.
However, for constants of more complex datatypes, e.g. nested packed
structures, the use of constant functions may infer Xs as (accidentally)
unassigned members will take their default values.
Default values are specified in IEEE1800-2017 Table 6-7.
This can be particularly subtle when a single member of a deeply nested packed
struct is wrongly declared with a 4-state type, e.g. `logic`, thus forcing all
other (previously 2-state) members to have a default value of `'X` instead of
the expected `'0`.

The equivalence operators ("case" equality/inequality) are written as 3
characters each (`===`, `!==`) and can only return false or true,
e.g. `4'b01XZ === 4'b01XZ` -> `1'b1` (true).
The partial equivalence operators ("logical" equality/inequality) are written
as 2 characters each (`==`, `!=`) and may return false, true, or unknown,
e.g. `4'b01XZ === 4'b01XZ` -> `1'bx` (unknown).

Let `w` be a 4-state signal which a systhesis tool will implement with a
collection of wires.
Let `c2` and `c4` be constants with 2-state and 4-state types respectively.
Without loss of generality, only the case/logical equality operators are
required to demonstrate troublesome expressions.

- `w === c2` Result may be false (`1'b0`), true (`1'b1`).
  If `w` contains any Xz or Zs, then the result is false (`1'b0`).
  This is *not* desired behavior as Xs in `w` are hidden and simulation is
  likely, but not certain, to mismatch synthesized hardware.
- `w === c4` Result may be false (`1'b0`), true (`1'b1`).
  If `w` contains any Xz or Zs, then the result is is true iff the constant
  `c4` has been defined with corresponding Xs and Zs.
  Comparison between unknown and unknown is all but certain, to mismatch
  synthesized hardware.
- `w == c2` Result may be false (`1'b0`), true (`1'b1`), or unknown (`1'bX`).
  If `w` contains any Xs or Zs, then the result is unknown.
  This is desired behavior as it sufficiently models synthesized physical
  hardware.
- `w == c4` Result may be false (`1'b0`), true (`1'b1`), or unknown (`1'bX`).
  If `c4` contains any Xs or Zs, then the result will always be unknown.
  While that may be noticed early in simulation, unwitting designers may be
  tempted to prevent X-propagation on the result, thus hiding any issues with
  Xs or Zs on `w`.

The use of 4-state constants with wildcard equality operators is a slightly
different usecase.
If wildcard equality operators are used with 4-state constants in your code,
this rule should be considered on a case-by-case basis.

See also:
  - **localparam_explicit_type** - Useful companion rule.
  - **parameter_explicit_type** - Useful companion rule.
  - **parameter_type_twostate** - Useful companion rule, equivalent reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 6.8 Variable declarations
  - 6.11 Integer data types
  - 7.2.1 Packed structures
  - 11.4.5 Equality operators
  - 11.4.6 Wildcard equality operators

NOTE: The reasoning behind this rule invites the creation of some new rules:
1. Check that members of a packed structure definition are either all 2-state
  or all 4-state.
2. Check for the use of case equality operators.
3. Check that functions are not declared with a 4-state type.
