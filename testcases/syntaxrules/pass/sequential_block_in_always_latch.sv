module M;
  always_latch
    if (foo) a <= b;

  always_latch
    if (foo) b <= y;
    else     b <= z;

  always_latch
    case (foo)
      one:     a <= x;
      two:     b <= y;
      default: c <= z;
    endcase
endmodule
