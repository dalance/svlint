module M
  ( I.i foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M_nonansi
  ( foo
  );
  I.i foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
