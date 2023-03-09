module M;
  always @(a, b, c) q1 <= d;

  always_ff @(a, b, c) q2 <= d;

  always @( a
          , b
          , c
          ) q3 <= d;

  always_ff @(a
            , b
            , c
            ) q4 <= d;

  initial begin
    z = y;
    @(posedge a, negedge b, edge c, d)
    z = x;
  end
endmodule
