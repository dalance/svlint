module M;
  sequence Mn3; // Identifier doesn't match default required regex (lowercase).
    @(posedge c) a ##1 b
  endsequence
endmodule
