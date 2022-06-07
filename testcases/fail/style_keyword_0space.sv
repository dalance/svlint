module A;
  always_comb
    case (a)
      123:
        b = c;
      default : // space between `default` and colon.
        b = d;
    endcase
  function foo ();
    for (;;)
      if (a) break  ; // spaces between `break` and semicolon.
  endfunction
endmodule
