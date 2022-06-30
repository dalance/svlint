module A;
  initial begin
    if (foo) begin: l_foo
      a = b;
    end   : l_foo           // spaces between `end` and colon.

    if (foo) begin
      a = c;
    end   else begin       // multiple spaces after `end`.
      a = d;
    end
  end   // multiple spaces then comment after `end`.
endmodule

