module M;
  initial begin
    mn3: // Identifier matches default required regex (lowercase).
      assert (p) else $error(); // Simple immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    mn3: // Identifier matches default required regex (lowercase).
      assert #0 (p) else $error(); // Deferred immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  mn3: // Identifier matches default required regex (lowercase).
    assert #0 (p) else $error(); // Deferred immmediate assertion item.
endmodule
