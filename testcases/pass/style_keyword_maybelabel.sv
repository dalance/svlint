module A;
endmodule: A // colon immediately after `endmodule`
package A;
    function foo();
    endfunction
//  ^^^^^^^^^^^ newline after `endfunction`
endpackage // 1 space then comment after `endpackage`

