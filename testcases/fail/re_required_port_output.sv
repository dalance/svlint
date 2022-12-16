module M
  ( output Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  output Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
