module M;
  always @* // Sensitive to `b` and `c`.
    a = b + c;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @(posedge clk) // Sensitive to edges of `clk`.
    q <= d;
endmodule
