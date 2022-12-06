module M;
  always_comb begin
    for (int i=0; i < 10; i++)
      a = 0;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb begin
    for (int i=0; i < 10; i++) a = 0; // This is okay.

    for (int i=0; i < 10; i++) // Catch any for-loop, not only the first.
      a = 0;
  end
endmodule
