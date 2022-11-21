module M;
  initial begin
    Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
      assert (p) else $error(); // Immmediate assertion.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
      assert #0 (p) else $error(); // Deferred immmediate assertion.
  end
endmodule
