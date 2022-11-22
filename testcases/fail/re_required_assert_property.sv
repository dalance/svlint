module M;
  Mn3: // Identifier doesn't match default required regex (lowercase).
    assert property (@(posedge c) p); // Concurrent assertion.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    Mn3: // Identifier doesn't match default required regex (lowercase).
      assert property (@(posedge c) p); // Concurrent assertion.
  end
endmodule
