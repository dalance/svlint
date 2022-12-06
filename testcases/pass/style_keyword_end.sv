module M;
  initial begin
    if (foo) begin: l_foo
      a = b;
    end: l_foo           // colon immediately after `end`.

    if (foo) begin
      a = c;
    end else begin       // 1 space after `end`.
      a = d;
    end
//  ^^^ newline after `end`.
  end // 1 space then comment after `end`.
endmodule

