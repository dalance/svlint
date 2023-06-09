module M;
  property Xfoo; // Identifier doesn't match default forbidden regex (X prefix).
    @(posedge c) p; // Concurrent assertion.
  endproperty
endmodule
