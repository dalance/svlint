module M;
  always_comb
    case (x)
      1: y = 0;
      default: y = 0;
    endcase

  always_ff @(clk)
    case (x)
      1: y = 0;
      default: y = 0;
    endcase
endmodule
