module  M;                  // multiple spaces after `module`.
  for(i = 0; i < 5; i++)    // no spaces after `for`.
    assign  foo = bar;      // multiple spaces after `assign`.
  always_ff@(posedge clk)   // no spaces after `always_ff`.
    if  (a)                 // multiple spaces after `if`.
      case(a)               // no spaces after `case`.
        1: foo <= bar;
      endcase
endmodule
