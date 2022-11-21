module M;
  initial begin
    mn3: // Identifier matches default required regex (lowercase).
      assert (p) else $error(); // Immmediate assertion.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    mn3: // Identifier matches default required regex (lowercase).
      assert #0 (p) else $error(); // Deferred immmediate assertion.
  end
endmodule
