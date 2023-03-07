module M;
  Packet p1 = new  ; // Spaces before semicolon.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  C1 p2 = new (1, 2, 3); // Spaces before parenthesis.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  C2 p3 = new// No space before comment.
    ( STARTUP
    , A_RATHER_LONG_CONSTANT_IDENTIFIER
    , 456
    );
endmodule
