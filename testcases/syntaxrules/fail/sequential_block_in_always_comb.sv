module M;
  always_comb begin
    a = z;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    if (bar) begin
      b = z;
    end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    if (bar) c = z;
    else begin
      c = z;
    end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    case (bar)
      one: begin
        d = z;
      end
      two: d = z;
      default: d = z;
    endcase
endmodule
