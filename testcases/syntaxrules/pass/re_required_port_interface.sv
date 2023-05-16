module M
  ( I.i mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  I.i mn3; // Identifier matches default required regex (lowercase).
endmodule
