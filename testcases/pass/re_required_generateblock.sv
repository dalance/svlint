module M;
  if (0) begin: mn3 // Identifier matches default required regex (lowercase).
    assign a = 0;
  end: mn3
  else begin: mn4 // Identifier matches default required regex (lowercase).
    assign a = 1;
  end: mn4

  // Identifier matches default required regex (lowercase).
  for (genvar i=0; i < 5; i++) begin: mn5
    assign b[i] = 0;
  end: mn5

  case (0)
    0: begin: mn6 // Identifier matches default required regex (lowercase).
      assign c = 0;
    end: mn6
    1: begin: mn7 // Identifier matches default required regex (lowercase).
      assign c = 1;
    end: mn7
    default: begin: mn8 // Identifier matches default required regex (lowercase).
      assign c = 2;
    end: mn8
  endcase
endmodule
