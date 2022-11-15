module M;
  localparam int P1 = a | b; // Single space around `|`.

  localparam int P2 =
    a &
    aMask; // Newline following `&`.

  localparam int P3 =
    a & // Single space then comment following `&`.
    aMask;
endmodule
