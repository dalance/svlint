module M;
  sequence foo; // Unconfigured forbidden regex matches (almost) anything.
    @(posedge c) a ##1 b
  endsequence
endmodule
