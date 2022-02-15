package P;
endpackage

interface I;
endinterface

module M;
  for (i = 0; i < 5; i++)
    assign foo = bar;

  always_ff @(posedge clk)
    if (a)
      case (a)
        1: foo <= bar;
      endcase
endmodule
