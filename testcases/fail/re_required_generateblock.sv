module M;
  if (0) begin: Mn3 // Identifier doesn't match default required regex (lowercase).
    assign a = 0;
  end: Mn3
  else begin: Mn4 // Identifier doesn't match default required regex (lowercase).
    assign a = 1;
  end: Mn4

  // Identifier doesn't match default required regex (lowercase).
  for (genvar i=0; i < 5; i++) begin: Mn5
    assign b[i] = 0;
  end: Mn5

  case (0)
    0: begin: Mn6 // Identifier doesn't match default required regex (lowercase).
      assign c = 0;
    end: Mn6
    1: begin: Mn7 // Identifier doesn't match default required regex (lowercase).
      assign c = 1;
    end: Mn7
    default: begin: Mn8 // Identifier doesn't match default required regex (lowercase).
      assign c = 2;
    end: Mn8
  endcase
endmodule
