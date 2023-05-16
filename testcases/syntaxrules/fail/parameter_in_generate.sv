module M;
  for (genvar i=0; i < 5; i++) begin
    parameter int P1 = 1;
  end

  if (1) begin
    parameter int P2 = 2;
  end else begin
    parameter int P3 = 3;
  end

  case (1)
    0: begin
      parameter int P4 = 4;
    end
    default: begin
      parameter int P5 = 5;
    end
  endcase
endmodule
