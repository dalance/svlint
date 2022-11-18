module M;
  always_comb
    case (a)
      123:
        b = c;
      default: // no space between `default` and colon.
        b = d;
    endcase

  function F;
    for (;;)
      if (a)
        break; // no space between `break` and semicolon.
  endfunction
endmodule
