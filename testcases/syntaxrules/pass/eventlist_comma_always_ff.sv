module M;
  always_ff @(posedge clk or posedge arst) q <= d;

  always_ff @( a
            or b
            or c
            ) q <= d;
endmodule
