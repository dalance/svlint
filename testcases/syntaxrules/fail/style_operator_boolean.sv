module M;
  localparam bit P1 = a&&b; // No space around `&&`.

  localparam bit P2 =
    a <
    b; // Newline after `<`.

  for (genvar i=0; i<5; i++) begin // No space around `<`.
  end
endmodule
