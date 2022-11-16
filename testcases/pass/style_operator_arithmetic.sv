module M;
  localparam bit [a-1:0] P1 = b; // No space around `-`.

  localparam int P2 = a + b; // Single space around `+`.

  localparam int P3 =
    a *
    b; // Newline following `*`.

  localparam int P4 =
    a * // Single space then comment following `*`.
    b;
endmodule
