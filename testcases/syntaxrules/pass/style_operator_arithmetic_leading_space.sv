module M;
  localparam int P2 = a + b; // Multiple spaces before `+`.

  // One space before `*`.
  localparam int P3 = a * b;

  // One space before `**`.
  localparam int P4 = a ** b;

  // One space before `+`.
  localparam int P5 = a + b;

  // One space before `%`.
  localparam int P6 = a % b;

  // One space before `/`.
  localparam int P7 = a / b;

  // When the previous expression is (`expr`) type
  localparam int P13 = (a + b) * c;
endmodule
