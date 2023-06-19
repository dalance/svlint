module M
  ( inout mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  inout mn3; // Identifier matches default required regex (lowercase).
endmodule
