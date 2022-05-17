module M;
  localparam bit  A = 0;  // multiple spaces after `bit`.
  localparam int
    B = 0;                // newline after `int`.
  logic // foo
    a;                    // single-line comment after `logic`.
  reg /* bar */ b;        // multi-line after `reg`.
  wire        c;          // multiple spaces after `wire`.
endmodule
