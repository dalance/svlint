The SystemVerilog language is specified in terms of simulation and allows
procedural statements to be used in both combinational (`always_comb`)
and sequential (`always_ff`, `always_latch`) logic processes.
The specification of logic with procedures facilitates straightforward
translation of algorithms which are previously modelled as procedures, e.g. an
algorithm described in a paper and demonstrated with a Python reference model.
Logic specified with procedures is also (often) synthesizable which makes this
a powerful language feature for quickly building a proof-of-concept
implementation, perhaps on an FPGA.
However, this language feature has several downsides for designs which are to
be trusted with large amounts of investment:
- Visualizing the expected logic with a schematic may be very difficult, thus
  leading to problems with routing and verification.
- Trivial-looking code can produce enormously complex logic.
- Trivial-looking changes can easily result in vastly different outcomes from
  synthesis.

A good mantra for synthesizable design is: If you find it easy to draw a
detailed schematic, then a synthesis tool will most likely produce a good
solution quickly.
For a production-worthy design, where you want to have full confidence in your
understanding of how the code works under all the various tools (synthesis,
LEC, simulation, formal proof, etc.), using only combinatial code to specifiy
combinational logic reduces the risk of mis-interpretations by different tools.
This is the same line of reasoning behing the `sequential_block_in_always_*`
rules.

See also:
- **loop_statement_in_always_ff** - Useful companion rule.
- **loop_statement_in_always_latch** - Useful companion rule.
- **sequential_block_in_always_comb** - Useful companion rule.
- **sequential_block_in_always_ff** - Useful companion rule.
- **sequential_block_in_always_latch** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 9.2.2 Always procedures
- 12.7 Loop statements
