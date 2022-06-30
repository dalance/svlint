module M;                   // 1 space after `module`.
  for (i = 0; i < 5; i++)   // 1 space after `for`.
    assign foo = bar;       // 1 space after `assign`.
  always_ff @(posedge clk)  // 1 space after `always_ff`.
    if (a)                  // 1 space after `if`.
      case (a)              // 1 space after `case`.
        1: foo <= bar;
      endcase
endmodule
