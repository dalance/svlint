module M;
  always @(posedge clk)
    assert (A)
      else $error("A should be high.");

  // Simulator must report line number, label, and time on each violation.
  asrt_b1: assert property (@(posedge clk) B1)
    else $error("B1 should be high.");
  asrt_b2: assert property (@(posedge clk) B2)
    else $error("B2 should be high.");
endmodule
