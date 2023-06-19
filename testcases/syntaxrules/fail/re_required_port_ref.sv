module M
  ( ref Mn3 // Identifier doesn't match default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( Mn3
  );
  ref var Mn3; // Identifier doesn't match default required regex (lowercase).
endmodule
