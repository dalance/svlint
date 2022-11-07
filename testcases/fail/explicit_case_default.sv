module M;
  always_comb
    case (x)
      1: y = 0; // Incompletely specified case implies memory.
    endcase

  always_ff @(clk) begin
    case (x)
      1: y = 0;
      default: y = 0; // Explicit default arm is good.
    endcase

    case (y)
      1: y = 0; // Implicit default arm.
    endcase
  end
endmodule
