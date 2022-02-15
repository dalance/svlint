package  P; // too many spaces
endpackage

interface
I; // newline, not a space
endinterface

module  M; // too many spaces
  for(i = 0; i < 5; i++) // missing space
    assign  foo = bar; // too many spaces

  always_ff @(posedge clk)
    if(a) // missing space
      case   (a) // too many spaces
        1: foo <= bar;
      endcase
endmodule
