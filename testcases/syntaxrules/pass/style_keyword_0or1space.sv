module M;
  function F;
    if (a)
      return; // semicolon immediately after `return`.
    else
      return a; // 1 space then expression after `return`.
  endfunction

  import "DPI-C" function bit bar();
endmodule
