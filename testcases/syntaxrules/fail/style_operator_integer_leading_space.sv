module M;
  localparam int P1 = a|b; // No space around `|`.

  localparam int P2 = a     & aMask; // Multiple spaces before `&`.
endmodule
