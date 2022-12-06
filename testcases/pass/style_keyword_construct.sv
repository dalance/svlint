module M;
  always_comb a = b;  // 1 space after `always_comb`.

  initial begin       // 1 space after `initial`.
    foo = bar;
  end

  always_latch
    if (a) b = c;     // newline after `always_latch`.
    else d = e;       // 1 space after `else`.

  final // 1 space then comment after `final`.
    foo = bar;
endmodule

