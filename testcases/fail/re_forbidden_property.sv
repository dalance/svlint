module M;
  property foo; // Unconfigured forbidden regex matches (almost) anything.
    @(posedge c) p; // Concurrent assertion.
  endproperty
endmodule
