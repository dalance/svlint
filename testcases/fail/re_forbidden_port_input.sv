module M
  ( input foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M_nonansi
  ( foo
  );
  input foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
