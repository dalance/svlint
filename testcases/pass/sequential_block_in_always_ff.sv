module a;
  always_ff @(posedge clk)
    d <= z;

  always_ff @(posedge clk)
    if (foo) e <= z;

  always_ff @(posedge clk)
    if (foo) f <= z;
    else     f <= z;

  always_ff @(posedge clk)
    case (foo)
      one:     g <= z;
      two:     g <= z;
      default: g <= z;
    endcase
endmodule
