module M
  ( I.i Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  I.i Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
