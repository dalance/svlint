module M
  ( ref Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  );
endmodule

module M_nonansi
  ( Xfoo
  );
  ref var Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
endmodule
