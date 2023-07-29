module M;
  localparam bit P1 = a && b; // One space before `&&`.

  for (genvar i=0; i < 5; i++) begin // One space around `<`.
  end
endmodule
