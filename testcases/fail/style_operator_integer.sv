module M;
  localparam int P1 = a|b; // No space around `|`.

  localparam int P2 =
    a &

    aMask; // Multiple newlines following `&`.

  localparam int P3 =
    a &  // Multiple spaces then comment following `&`.
    aMask;
endmodule
