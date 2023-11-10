module M
  ( a
  , b
  );
  input  a;   // Declaring ports outside the module header declaration
  output b;   // makes this a non-ANSI module.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;     // A module with no portlist is ANSI, but allowed.
endmodule
