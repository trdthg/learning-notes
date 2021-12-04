module foo (
input a,
input b,
input c,
output o
);

assign o = (a & b) | c;

endmodule
