module M;
  always_latch if (a === b) z = y;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb z = (a !== b) ? y : x;
endmodule