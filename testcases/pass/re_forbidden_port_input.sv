module M
  ( input Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  input Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
