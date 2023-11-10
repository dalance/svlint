module M;
  always @*
    if (a == b) // Logical-equality operator is not an assignment.
      z = y;    // Simple assignment operator is allowed.
endmodule
