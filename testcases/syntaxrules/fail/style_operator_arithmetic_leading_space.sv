module M;
  localparam int P2 = a  + b; // Multiple spaces before `+`.

  // No space before `*`.
  localparam int P3 = a* b;

  // No space before `**`.
  localparam int P4 = a** b;

  // No space before `+`.
  localparam int P5 = a+ b;

  // No space before `%`.
  localparam int P6 = a% b;

  // No space before `/`.
  localparam int P7 = a/ b;

  // Multiple spaces before `+`.
  localparam int P8 = a  + b;

  // Multiple spaces before `*`.
  localparam int P9 = a  * b;

  // Multiple spaces before `**`.
  localparam int P10 = a  ** b;

  // Multiple spaces before `%`.
  localparam int P11 = a  % b;

  // Multiple spaces before `/`.
  localparam int P12 = a  / b;

  // When the previous expression is (`expr`) type
  localparam int P13 = (a + b)    * c;
endmodule
