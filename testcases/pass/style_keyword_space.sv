package A;
endpackage

interface A;
endinterface

module A;
  for (i = 0; i < 5; i++)
    assign foo = bar;

  always_ff @(posedge clk)
    if (a)
      case (a)
        1: foo <= bar;
      endcase
endmodule
