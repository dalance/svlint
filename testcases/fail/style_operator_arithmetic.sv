module M;
  localparam int P2 = a  +  b; // Multiple spaces around `+`.

  localparam int P3 =
    a *

    b; // Multiple newlines following `*`.

  localparam int P4 =
    a *  // Multiple spaces then comment following `*`.
    b;
endmodule
