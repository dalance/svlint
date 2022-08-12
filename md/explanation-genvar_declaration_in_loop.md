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

Although it is not explicitly stated, a reasonable interpretation is that a
genvar declared inside a generate loop may only be used within that specific
loop generate construct, i.e. locally scoped.
This interpretation matches C99 (ISO/IEC 9899:1999), while a requirement for
the genvar to be declared outside would match ANSI C (ISO/IEC 9899:1990).
This rule checks that genvars are declared in a C99-like style so that the
identifier is declared beside its use which has several advantages:
  - The purpose of the genvar is immediately clear, e.g. it is easy to read
    that the `i` in `for (genvar i=0; i < N_BITS; i++) ...` refers to a bit
    index.
    In contrast, `genvar j; ...many lines... for (j=0; j < N_BITS; j++) ...`
    requires the reader to keep `j` in their head for a longer time.
  - Only one comment is necessary, rather than splitting or duplicating the
    information.
  - When a future revision of your code removes a generate loop, the genvar
    declaration is implictly removed too, which avoids lingering useless and
    distracting statements.
  - A subsequent generate loop cannot accidentally use a "leftover" genvar
    which is intended for use only by a previous generate loop.
    The LRM only requires that "A genvar shall not be referenced anywhere other
    than in a loop generate scheme.".

Given the lack of clarity in the LRM, it is unsurprising that some tools might
not support both ways of declaring genvars, so the related rule
**genvar_declaration_out_loop** assumes a stricter interpretation of the LRM
and checks that declarations must be separate from the generate loop syntax.

See also:
  - **genvar_declaration_out_loop** - Opposite reasoning.

The most relevant clauses of IEEE1800-2017 are:
  - 27.4 Loop generate constructs
