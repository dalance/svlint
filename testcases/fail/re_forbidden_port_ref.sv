module M
  ( ref foo // Unconfigured forbidden regex matches (almost) anything.
  );
endmodule

module M_nonansi
  ( foo
  );
  ref var foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
