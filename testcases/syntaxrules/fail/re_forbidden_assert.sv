module M;
  initial begin
    foo: // Unconfigured forbidden regex matches (almost) anything.
      assert (p) else $error(); // Simple immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    foo: // Unconfigured forbidden regex matches (almost) anything.
      assert #0 (p) else $error(); // Deferred immmediate assertion statement.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  foo: // Unconfigured forbidden regex matches (almost) anything.
    assert #0 (p) else $error(); // Deferred immmediate assertion item.
endmodule
