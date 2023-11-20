module M      // An ANSI module has ports declared in the module header.
  ( input  a
  , output b
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;     // A module with no ports is also ANSI.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M      // Declaring ports in the header with default direction (inout)
  ( a         // also specifies an ANSI module.
  , b
  );
endmodule
