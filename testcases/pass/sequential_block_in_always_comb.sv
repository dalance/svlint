module a;
  always_comb
    e = z;

  always_comb
    if (foo) f = z;
    else     f = z;

  always_comb
    case (foo)
      one:     g = z;
      two:     g = z;
      default: g = z;
    endcase
endmodule
