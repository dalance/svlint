module M;
  always_latch begin
    a <= z;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_latch
    if (x) begin
      a <= z;
    end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_latch
    if (x) a <= z;
    else begin
      a <= z;
    end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_latch
    case (x)
      foo: begin
        a <= z;
      end
      bar: a <= z;
      default: a <= z;
    endcase
endmodule
