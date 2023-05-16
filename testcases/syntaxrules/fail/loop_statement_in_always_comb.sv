module M;

  always_comb
    for (int i = 0; i < 5; i++)
      if (0 == i)
        a = f();
      else
        a = a + 5;

endmodule
////////////////////////////////////////////////////////////////////////////////
module M;

  always_comb
    if (x)
      for (int i = 0; i < 5; i++)
        a[i] = a + 5;
    else
      for (int i = 0; i < 5; i++)
        a = b[i] + 5;

endmodule
