module M
  ( inout foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  inout foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
