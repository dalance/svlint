module M;
  if (0) begin: Xfoo // Identifier doesn't match default forbidden regex (X prefix).
    assign a = 0;
  end: Xfoo
  else begin: Xbar // Identifier doesn't match default forbidden regex (X prefix).
    assign a = 1;
  end: Xbar
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  // Identifier doesn't match default forbidden regex (X prefix).
  for (genvar i=0; i < 5; i++) begin: Xfoo
    assign b[i] = 0;
  end: Xfoo
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  case (0)
    0: begin: Xfoo // Identifier doesn't match default forbidden regex (X prefix).
      assign c = 0;
    end: Xfoo
    1: begin: Xbar // Identifier doesn't match default forbidden regex (X prefix).
      assign c = 1;
    end: Xbar
    default: begin: Xbaz // Identifier doesn't match default forbidden regex (X prefix).
      assign c = 2;
    end: Xbaz
  endcase
endmodule
