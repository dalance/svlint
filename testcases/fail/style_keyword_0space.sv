module M;
  always_comb
    case (a)
      123:
        b = c;
      default : // Space between `default` and colon.
        b = d;
    endcase
  function foo ();
    for (;;)
      if (a) break  ; // Spaces between `break` and semicolon.
  endfunction
endmodule
