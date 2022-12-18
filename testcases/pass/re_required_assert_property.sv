module M;
  mn3: // Identifier matches default required regex (lowercase).
    assert property (@(posedge c) p); // Concurrent assertion.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    mn3: // Identifier matches default required regex (lowercase).
      assert property (@(posedge c) p); // Concurrent assertion.
  end
endmodule
