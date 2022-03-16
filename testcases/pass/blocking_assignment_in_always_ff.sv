module M;
always_ff @(posedge clk) q1 <= d; // Correct.

/* svlint off blocking_assignment_in_always_ff */
always_ff @(posedge clk) q2 = d;  // Control comments avoid failure.
/* svlint on blocking_assignment_in_always_ff */
endmodule
