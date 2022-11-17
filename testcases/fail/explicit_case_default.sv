module M;
  always_comb
    case (x)
      1: a = 0; // Incompletely specified case implies memory.
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_ff @(clk) begin
    case (x)
      1: a = 0;
      default: a = 0; // Explicit default arm is good.
    endcase

    case (y)
      1: b = 0; // Implicit default arm.
    endcase
  end
endmodule
