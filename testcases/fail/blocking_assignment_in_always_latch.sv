module M;
  always_latch
    if (load)
      q = d;
endmodule
