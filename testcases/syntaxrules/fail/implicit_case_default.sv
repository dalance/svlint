module M;
  always_comb
    case (x)
      1: a = 0; // No implicit or explicit case default
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb begin
    a = 0;
    case(x)
      1: b = 0;
    endcase
  end
endmodule
