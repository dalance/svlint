# Naming

A set of lightweight naming-only checks which are "suggested" in the
explanations of the **prefix** rules.

## Motivation

Naming conventions help a human reader to take in large amounts of detailed
information (e.g. from netlists, timing reports) by allowing the reader to
predict the function of a signal from its name, and predict part of a signal's
name from its function.
For example, a common convention is: "All signals inferring the output of a
flip-flop must be suffixed with `_q`."
If an engineer reads a synthesized netlist and sees a flip-flop cell named
without the `_q` suffix, there may be a coding error, so further checks
with the author are required.
On the frontend, a reader can quickly scan a SystemVerilog file to check that
signals have the `_q` suffix if (and only if) they are driven in `always_ff`
processes.
Other naming conventions are useful for helping readers follow the direction of
signals through ports, find files quickly in a large filesystem, find
appropriate files from hierarchical paths in a netlist, and more.

TODO
