module M;
  Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
    assert property (@(posedge c) p); // Concurrent assertion.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    Xfoo: // Identifier doesn't match default forbidden regex (X prefix).
      assert property (@(posedge c) p); // Concurrent assertion.
  end
endmodule
