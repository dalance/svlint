module M;
  sequence mn3; // Identifier matches default required regex (lowercase).
    @(posedge c) a ##1 b
  endsequence
endmodule
