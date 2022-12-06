module M;
  always_comb
    if (a)
      a = 0; // Missing begin/end.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    if (a) begin
      a = 0;
    end else if (a)
      a = 0; // Missing begin/end.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    if (a) begin
      a = 0;
    end else if (a) begin
      a = 0;
    end else
      a = 0; // Missing begin/end.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb begin
    if (a)
      a = 0; // Missing begin/end.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb begin
    if (a) a = 0; // This conditional statement is okay.
    else if (a) a = 0;
    else a = 0;

    if (a)   // Check all if-statements, not only the first.
      a = 0; // Missing begin/end.
  end
endmodule
