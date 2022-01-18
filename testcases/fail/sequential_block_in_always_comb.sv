module a;
  always_comb begin
    a = z;
  end

  always_comb
    if (bar) begin
      b = z;
    end

  always_comb
    if (bar) c = z;
    else begin
      c = z;
    end

  always_comb
    case (bar)
      one: begin
        d = z;
      end
      two: d = z;
      default: d = z;
    endcase
endmodule
