module M;
  initial begin
    Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
      assert (p) else $error(); // Simple immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
      assert #0 (p) else $error(); // Deferred immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
    assert #0 (p) else $error(); // Deferred immmediate assertion item.
endmodule
