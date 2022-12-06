module M;
  always_comb
    a = b;

  always_comb
    if (x)
      a = b;
    else
      a = c;

  always_comb
    case (x)
      one:     a = x;
      two:     a = y;
      default: a = z;
    endcase
endmodule
