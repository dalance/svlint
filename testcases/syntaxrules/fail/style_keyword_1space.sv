module  M;                  // Multiple spaces after `module`.
  for(genvar i = 0; i < 5; i++)    // No spaces after `for`.
    assign  a = b;      // Multiple spaces after `assign`.
  always_ff@(posedge clk)   // No spaces after `always_ff`.
    if  (a)                 // Multiple spaces after `if`.
      case(a)               // No spaces after `case`.
        1: a <= b;
      endcase
endmodule
