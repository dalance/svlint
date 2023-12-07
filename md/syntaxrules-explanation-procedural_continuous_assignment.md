SystemVerilog continuous assignment (`assign x = y`) infers combinatorial logic
that continuously drives the LHS and changes with any change on the RHS.

Such construct in a procedural (`always*`) block which is only triggered
on the changes of the signals in the sensitivity list may not be synthesizable.
