module M
  ( output foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  output foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
