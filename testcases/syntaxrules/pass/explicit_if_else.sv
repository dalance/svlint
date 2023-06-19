module M;
  always_ff @(clk)
    if (x) y <= 0;
    else   y <= z;

  always_comb
    if (x) y = 0;
    else   y = z;
endmodule
