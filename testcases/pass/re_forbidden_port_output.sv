module M
  ( output Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M_nonansi
  ( Xfoo
  );
  output Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
