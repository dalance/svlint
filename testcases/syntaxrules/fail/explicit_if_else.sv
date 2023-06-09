module M;
  always_comb
    if (x) y = 0; // Incompletely specified condition implies memory.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_ff @(clk) begin
    if (a)
      b <= c;
    else // Explicit else clause is good.
      b <= d;

    if (b)
      c <= d; // Implicit else clause.
  end
endmodule
