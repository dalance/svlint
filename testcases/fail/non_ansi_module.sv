module M
  ( a
  , b
  );
  input  a;   // Declaring ports outside the module header declaration
  output b;   // makes this a non-ANSI module.
endmodule
