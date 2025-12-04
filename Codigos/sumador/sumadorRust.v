

module Sumador(a,b,sum);
    
    // Module arguments
    input wire signed [7:0] a;
    input wire signed [7:0] b;
    output reg signed [8:0] sum;
    
    // Update code
    always @(*) begin
        sum = $signed(a) + $signed(b);
    end
    
endmodule // top
