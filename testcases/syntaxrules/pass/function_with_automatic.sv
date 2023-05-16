module M;
  function automatic F;
  endfunction
endmodule

module automatic M; // Default lifetime.
  function F;
  endfunction
endmodule

interface automatic I;
  function F;
  endfunction
endinterface

program automatic P;
  function F;
  endfunction
endprogram

package automatic P;
  function F;
  endfunction
endpackage

module static M;
  function automatic F; // Override default lifetime.
  endfunction
endmodule

interface static I;
  function automatic F;
  endfunction
endinterface

program static P;
  function automatic F;
  endfunction
endprogram

package static P;
  function automatic F;
  endfunction
endpackage

module M;
  class C;
    function F; // Function in class is automatic.
    endfunction
  endclass
endmodule

module automatic M;
  class C;
    function F;
    endfunction
  endclass
endmodule

module static M;
  class C;
    function F;
    endfunction
  endclass
endmodule
