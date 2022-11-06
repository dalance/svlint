module M
  ( ref mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  ref var mn3; // Identifier matches default required regex (lowercase).
endmodule
