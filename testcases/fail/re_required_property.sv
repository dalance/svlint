module M;
  property Mn3; // Identifier doesn't match default required regex (lowercase).
    @(posedge c) p; // Concurrent assertion.
  endproperty
endmodule
