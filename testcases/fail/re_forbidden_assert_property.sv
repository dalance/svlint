module M;
  foo: // Unconfigured forbidden regex matches (almost) anything.
    assert property (@(posedge c) p); // Concurrent assertion.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial begin
    foo: // Unconfigured forbidden regex matches (almost) anything.
      assert property (@(posedge c) p); // Concurrent assertion.
  end
endmodule
