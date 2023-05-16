module M;

  for (genvar i = 0; i < 5; i++) begin
    if (0 == i) begin
      always_latch if (load) a[0] <= f();
    end else begin
      always_latch if (load) a[i] <= a[i-1] + 5;
    end
  end

endmodule
