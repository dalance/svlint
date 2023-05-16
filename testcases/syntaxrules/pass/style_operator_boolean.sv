module M;
  localparam bit P1 = a && b; // Single space around `&&`.

  localparam bit P2 = a < b; // Single space around `<`.

  for (genvar i=0; i < 5; i++) begin // Single space around `<`.
  end
endmodule
