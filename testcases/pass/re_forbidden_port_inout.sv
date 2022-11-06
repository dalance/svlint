module M
  ( inout Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  inout Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
