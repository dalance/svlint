module M;
  always_latch if (a == b) z = y;

  always_comb z = (a != b) ? y : x;

  always_latch if (a ==? b) z = y;

  always_comb z = (a !=? b) ? y : x;
endmodule
