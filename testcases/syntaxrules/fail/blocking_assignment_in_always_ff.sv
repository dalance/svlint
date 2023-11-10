module M;
  always_ff @(posedge clk) q = d; // Failure.
endmodule
