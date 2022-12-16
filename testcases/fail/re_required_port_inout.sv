module M
  ( inout Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  inout Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
