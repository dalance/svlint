//module M;
//  initial begin
//    Mn3: // Identifier doesn't match default required regex (lowercase).
//      assert (p) else $error(); // Simple immmediate assertion statement.
//  end
//endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    Mn3: // Identifier doesn't match default required regex (lowercase).
      assert #0 (p) else $error(); // Deferred immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
//module M;
//  Mn3: // Identifier doesn't match default required regex (lowercase).
//    assert #0 (p) else $error(); // Deferred immmediate assertion item.
//endmodule
