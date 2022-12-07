module M;
  genvar Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  // Identifier doesn't match default forbidden regex (X prefix).
  for (genvar Xbar=0; Xbar < 5; Xbar++) begin
  end
endmodule
