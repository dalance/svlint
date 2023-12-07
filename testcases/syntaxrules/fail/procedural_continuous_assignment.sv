module M (
    input logic clk,
    input logic a,
    input logic b,
    output logic c
);

always_ff @(posedge clk)
    assign c = a + b;

endmodule
