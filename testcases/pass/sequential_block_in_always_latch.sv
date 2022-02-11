module a;
  always_latch
    if (foo) e <= z;

  always_latch
    if (foo) f <= z;
    else     f <= z;

  always_latch
    case (foo)
      one:     g <= z;
      two:     g <= z;
      default: g <= z;
    endcase
endmodule
