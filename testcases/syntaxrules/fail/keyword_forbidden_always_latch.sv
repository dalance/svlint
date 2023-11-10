module M;
  always_latch
    if (en)
      d <= q;
endmodule
