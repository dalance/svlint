module M
  ( input mn3 // Identifier matches default required regex (lowercase).
  );
endmodule

module M_nonansi
  ( mn3
  );
  input mn3; // Identifier matches default required regex (lowercase).
endmodule
