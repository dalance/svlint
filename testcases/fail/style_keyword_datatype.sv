module M;
  localparam bit  A = 0;  // Multiple spaces after `bit`.
  localparam int
    B = 0;                // Newline after `int`.
  logic // foo
    a;                    // Single-line comment after `logic`.
  reg /* bar */ b;        // Multi-line after `reg`.
  wire        c;          // Multiple spaces after `wire`.
endmodule
