module M;
  always @(posedge clk)
    assert (A) // These are legal, but potentially confusing.
    else begin
      $display("A should be high."); // Write to STDOUT.

      // Update global variable.
      errorId = 5; // What value if multiple immediate assertions fail?
      errorCount++; // Hopefully simulator blocks between processes.
    end

  // In what order do these action blocks occur?
  asrt_b1: assert property (@(posedge clk) B1)
    else begin
      $display("B1 should be high.");
      errorId = 1;
      errorCount++;
    end;
  asrt_b2: assert property (@(posedge clk) B2)
    else begin
      $display("B2 should be high.");
      errorId = 2;
      errorCount++;
    end;
endmodule
