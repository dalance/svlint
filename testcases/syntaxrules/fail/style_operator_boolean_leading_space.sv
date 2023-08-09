module M;
  localparam bit P1 = a&&b; // No space before `&&`.

  localparam bit P2 = a   < b; // Multiple spaces after `<`.

  for (genvar i=0; i<5; i++) begin // No space around `<`.
  end
endmodule
