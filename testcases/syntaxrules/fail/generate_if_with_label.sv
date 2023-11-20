module M;
  if (x)                        // No begin/end delimiters.
    assign a = 0;               // if condition.
  else if (x) begin: l_def
    assign a = 1;
  end else begin: l_hij
    assign a = 2;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin: l_abc
    assign a = 0;
  end else if (x)               // No begin/end delimiters.
    assign a = 1;               // else-if condition.
  else begin: l_hij
    assign a = 2;
  end
endmodule

// TODO: This isn't caught.
module M;
  if (x) begin: l_abc
    assign a = 0;
  end else if (x) begin: l_def
    assign a = 1;
  end else                      // No begin/end delimiters.
    assign a = 2;               // else condition
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin                  // begin/end delimiters, but no label.
    assign a = 0;               // if condition.
  end else if (x) begin: l_def
    assign a = 1;
  end else begin: l_hij
    assign a = 2;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin: l_abc
    assign a = 0;
  end else if (x) begin         // begin/end delimiters, but no label.
    assign a = 1;               // else-if condition.
  end else begin: l_hij
    assign a = 2;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin: l_abc
    assign a = 0;
  end else if (x) begin: l_def
    assign a = 1;
  end else begin                // begin/end delimiters, but no label.
    assign a = 2;               // else condition
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin: foo             // With label, but no prefix.
    assign a = 0;               // if condition.
  end else if (x) begin: l_def
    assign a = 1;
  end else begin: l_hij
    assign a = 2;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin: l_abc
    assign a = 0;
  end else if (x) begin: foo    // With label, but no prefix.
    assign a = 1;               // else-if condition.
  end else begin: l_hij
    assign a = 2;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (x) begin: l_abc
    assign a = 0;
  end else if (x) begin: l_def
    assign a = 1;
  end else begin: foo           // With label, but no prefix.
    assign a = 2;               // else condition
  end
endmodule
