module M;
  always_comb   a = b;  // Multiple spaces after `always_comb`.
  initial     begin       // Multiple spaces after `initial`.
    a = b;
  end
  always_latch
    if (a) b = c;
    else      d = e;  // Multiple spaces after `else`.
  final  // Multiple spaces then comment after `final`.
    a = b;
endmodule

