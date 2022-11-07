The specification of genvar declarations in IEEE1800-2017 is not
straightforward.
The formal syntax of `genvar_initialization` specified in Annex A.4.2 (Generated
instantiation) suggests that the `genvar` keyword is optional, but the second
sentence of Clause 27.5 declares that
"The loop index variable shall be declared in a genvar declaration prior to
its use in a loop generate scheme".
All 5 examples in Clause 27.4 (Loop generate constructs) declare the genvars
outside of the generate loops, and the formal syntax of `genvar_declaration` in
A.2.1.3 (Type declarations) is only applicable to declarations outside of
loop generate constructs.
That is, using syntax like `genvar i; for (i=0; ...)`.
However, several examples of declarations inside loop generate constructs are
present in other areas of the LRM like `for (genvar i=0; ...`:
- Clause 11.12 Let construct, example d, page 295.
- Clause 16.14.6.1 Arguments to procedural concurrent assertions, page 464.
- Clause 20.11 Elaboration system tasks, page 607.
- Clause 23.3.3.5 Unpacked array ports and arrays of instances, page 717.

This rule assumes a strict interpretation of the LRM and checks that
declarations must be separate from the generate loop syntax.

The related rule **genvar_declaration_in_loop** checks the opposite way because
C99-like declarations inside loop generate constructs can lead to code which is
easier to read and review.

See also:
  - **genvar_declaration_in_loop** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.4 Loop generate constructs
