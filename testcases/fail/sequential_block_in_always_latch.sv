module a;
  always_latch begin
    a <= z;
  end

  always_latch
    if (bar) begin
      b <= z;
    end

  always_latch
    if (bar) c <= z;
    else begin
      c <= z;
    end

  always_latch
    case (bar)
      one: begin
        d <= z;
      end
      two: d <= z;
      default: d <= z;
    endcase
endmodule
