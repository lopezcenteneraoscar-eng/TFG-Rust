module multiplicacion_Matrices (
    input rst,

    input signed [3:0] a00, a01,
    input signed [3:0] a10, a11,

    input signed [3:0] b00, b01,
    input signed [3:0] b10, b11, 

    output reg signed [31:0] c00, c01,
    output reg signed [31:0] c10, c11

);

    always @(*) begin
        if (rst) begin
            c00 <= 2'd0; c01 <= 2'd0;
            c10 <= 2'd0; c11 <= 2'd0;
        end else begin
           
            c00 <= a00*b00 + a01*b10;
            c01 <= a00*b01 + a01*b11;

            c10 <= a10*b00 + a11*b10;
            c11 <= a10*b01 + a11*b11;   
        end
    end
endmodule

