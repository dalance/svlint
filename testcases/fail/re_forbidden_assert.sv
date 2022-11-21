module M;
  initial begin
    foo: // Unconfigured forbidden regex matches (almost) anything.
      assert (p) else $error(); // Immmediate assertion.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    foo: // Unconfigured forbidden regex matches (almost) anything.
      assert #0 (p) else $error(); // Deferred immmediate assertion.
  end
endmodule
