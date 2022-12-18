module M;
  sequence Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
    @(posedge c) a ##1 b
  endsequence
endmodule
