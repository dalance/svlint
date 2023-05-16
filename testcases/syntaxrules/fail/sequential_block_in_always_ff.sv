module M;
  always_ff @(posedge clk) begin
    a <= z;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_ff @(posedge clk)
    if (x) begin
      a <= z;
    end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_ff @(posedge clk)
    if (x) a <= z;
    else begin
      a <= z;
    end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_ff @(posedge clk)
    case (x)
      foo: begin
        a <= z;
      end
      bar: a <= z;
      default: a <= z;
    endcase
endmodule
