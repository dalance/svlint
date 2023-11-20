module M      // An ANSI module has ports declared in the module header.
  ( input  a
  , output b
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M      // Declaring ports in the header with default direction (inout)
  ( a         // also specifies an ANSI module where directions are not given
  , b         // later.
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M
  ();         // A module with an empty portlist is ANSI.
endmodule
