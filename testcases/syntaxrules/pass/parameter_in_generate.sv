module M;
  for (genvar i=0; i < 5; i++) begin
    localparam int P1 = 1;
  end

  if (1) begin
    localparam int P2 = 2;
  end else begin
    localparam int P3 = 3;
  end

  case (1)
    0: begin
      localparam int P4 = 4;
    end
    default: begin
      localparam int P5 = 5;
    end
  endcase
endmodule
