
`begin_keywords "1800-2017"
`end_keywords
`celldefine
`endcelldefine
`unconnected_drive pull0
`nounconnected_drive
`pragma foo
`timescale 1ns / 1ps
`default_nettype none
`line 5 "foo.sv" 0
`resetall
/* This FILE is `__FILE__ */
/* This LINE is `__LINE__ */
// This directive is commented `celldefine
module testmodule(); //Those directives are also commented `ifdef FOO Foo `else Bar `endif
endmodule
`include "testcases/syntaxrules/pass/blocking_assignment_in_always_ff.sv"
`define FOO 5
`ifdef FOO
`elsif BAR
`else
`endif
`ifndef BAZ
`endif
`undef FOO
`undefineall
`ifdef FOO // `ifdef BAR
`endif
