module M;
  always @*
    if (en)
      d <= q;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @(en)
    if (en)
      d <= q;
endmodule
