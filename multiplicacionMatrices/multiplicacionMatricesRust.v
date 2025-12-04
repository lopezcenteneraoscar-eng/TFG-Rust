

module multiplicacionMatrices(rst,a$0,a$1,a$2,a$3,b$0,b$1,b$2,b$3,result$0,result$1,result$2,result$3);
    
    // Module arguments
    input wire  rst;
    input wire signed [3:0] a$0;
    input wire signed [3:0] a$1;
    input wire signed [3:0] a$2;
    input wire signed [3:0] a$3;
    input wire signed [3:0] b$0;
    input wire signed [3:0] b$1;
    input wire signed [3:0] b$2;
    input wire signed [3:0] b$3;
    output reg signed [31:0] result$0;
    output reg signed [31:0] result$1;
    output reg signed [31:0] result$2;
    output reg signed [31:0] result$3;
    
    // Update code
    always @(*) begin
        if (rst) begin
            result$0 = 32'h0;
            result$1 = 32'h0;
            result$2 = 32'h0;
            result$3 = 32'h0;
        end
        else begin
            result$0 = ($signed(a$0) * $signed(b$0)) + ($signed(a$1) * $signed(b$2));
            result$1 = ($signed(a$0) * $signed(b$1)) + ($signed(a$1) * $signed(b$3));
            result$2 = ($signed(a$2) * $signed(b$0)) + ($signed(a$3) * $signed(b$2));
            result$3 = ($signed(a$2) * $signed(b$1)) + ($signed(a$3) * $signed(b$3));
        end
    end
    
endmodule // top
