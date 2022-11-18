module M;
  initial begin
    if (foo) begin: l_foo
      a = b;
    end   : l_foo           // Spaces between `end` and colon.

    if (foo) begin
      a = c;
    end   else begin       // Multiple spaces after `end`.
      a = d;
    end
  end   // Multiple spaces then comment after `end`.
endmodule

