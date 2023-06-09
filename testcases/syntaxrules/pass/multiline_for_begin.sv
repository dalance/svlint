module M;
  always_comb begin
    for (int a=0; a < 10; a++) begin
      a = 0;
    end

    for (int a=0; a < 10; a++) a = 0;
  end
endmodule
