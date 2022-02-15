module A;
always_ff begin
    x <= 0;
end
/* svlint off blocking_assignment_in_always_ff */
always_ff begin
    x = 0;
end
/* svlint on blocking_assignment_in_always_ff */
endmodule
