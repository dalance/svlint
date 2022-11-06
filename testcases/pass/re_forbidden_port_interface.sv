module M
  ( I Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  I.i Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
