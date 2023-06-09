module M
  ( output mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  output mn3; // Identifier matches default required regex (lowercase).
endmodule
