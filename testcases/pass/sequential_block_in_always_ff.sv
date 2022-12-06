module M;
  always_ff @(posedge clk)
    q <= d;

  always_ff @(posedge clk)
    if (x) q <= d;

  always_ff @(posedge clk)
    if (rst) q <= 0;
    else     q <= d;

  always_ff @(posedge clk)
    case (foo)
      one:     q <= x;
      two:     r <= y;
      default: s <= z;
    endcase
endmodule
