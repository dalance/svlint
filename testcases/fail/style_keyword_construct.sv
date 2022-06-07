module A;
  always_comb   a = b;  // multiple spaces after `always_comb`.
  initial     begin       // multiple spaces after `initial`.
    foo = bar;
  end
  always_latch
    if (a) b = c;
    else      d = e;  // multiple spaces after `else`.
  final  // multiple spaces then comment after `final`.
    foo = bar;
endmodule

