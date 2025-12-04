

module ArbolSumadoresSegmentacion(inputs$0,inputs$1,inputs$2,inputs$3,inputs$4,inputs$5,inputs$6,inputs$7,result,clk,rstn);
    
    // Module arguments
    input wire signed [7:0] inputs$0;
    input wire signed [7:0] inputs$1;
    input wire signed [7:0] inputs$2;
    input wire signed [7:0] inputs$3;
    input wire signed [7:0] inputs$4;
    input wire signed [7:0] inputs$5;
    input wire signed [7:0] inputs$6;
    input wire signed [7:0] inputs$7;
    output reg signed [10:0] result;
    input wire  clk;
    input wire  rstn;
    
    // Stub signals
    reg signed [8:0] sum1$d;
    wire signed [8:0] sum1$q;
    reg  sum1$clock;
    reg signed [8:0] sum2$d;
    wire signed [8:0] sum2$q;
    reg  sum2$clock;
    reg signed [8:0] sum3$d;
    wire signed [8:0] sum3$q;
    reg  sum3$clock;
    reg signed [8:0] sum4$d;
    wire signed [8:0] sum4$q;
    reg  sum4$clock;
    reg signed [9:0] sum1_1$d;
    wire signed [9:0] sum1_1$q;
    reg  sum1_1$clock;
    reg signed [9:0] sum1_2$d;
    wire signed [9:0] sum1_2$q;
    reg  sum1_2$clock;
    reg signed [10:0] final_sum$d;
    wire signed [10:0] final_sum$q;
    reg  final_sum$clock;
    
    // Sub module instances
    top$sum1 sum1(
        .d(sum1$d),
        .q(sum1$q),
        .clock(sum1$clock)
    );
    top$sum2 sum2(
        .d(sum2$d),
        .q(sum2$q),
        .clock(sum2$clock)
    );
    top$sum3 sum3(
        .d(sum3$d),
        .q(sum3$q),
        .clock(sum3$clock)
    );
    top$sum4 sum4(
        .d(sum4$d),
        .q(sum4$q),
        .clock(sum4$clock)
    );
    top$sum1_1 sum1_1(
        .d(sum1_1$d),
        .q(sum1_1$q),
        .clock(sum1_1$clock)
    );
    top$sum1_2 sum1_2(
        .d(sum1_2$d),
        .q(sum1_2$q),
        .clock(sum1_2$clock)
    );
    top$final_sum final_sum(
        .d(final_sum$d),
        .q(final_sum$q),
        .clock(final_sum$clock)
    );
    
    // Update code
    always @(*) begin
        sum1$clock = clk;
        sum2$clock = clk;
        sum3$clock = clk;
        sum4$clock = clk;
        sum1_1$clock = clk;
        sum1_2$clock = clk;
        final_sum$clock = clk;
        if (~rstn) begin
            sum1$d = 32'h0;
            sum2$d = 32'h0;
            sum3$d = 32'h0;
            sum4$d = 32'h0;
            sum1_1$d = 32'h0;
            sum1_2$d = 32'h0;
            final_sum$d = 32'h0;
            result = 32'h0;
        end
        else begin
            sum1$d = $signed(inputs$0) + $signed(inputs$1);
            sum2$d = $signed(inputs$2) + $signed(inputs$3);
            sum3$d = $signed(inputs$4) + $signed(inputs$5);
            sum4$d = $signed(inputs$6) + $signed(inputs$7);
            sum1_1$d = $signed(sum1$q) + $signed(sum2$q);
            sum1_2$d = $signed(sum3$q) + $signed(sum4$q);
            final_sum$d = $signed(sum1_1$q) + $signed(sum1_2$q);
            result = final_sum$q;
        end
    end
    
endmodule // top


module top$final_sum(d,q,clock);
    
    // Module arguments
    input wire signed [10:0] d;
    output reg signed [10:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 11'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$final_sum


module top$sum1(d,q,clock);
    
    // Module arguments
    input wire signed [8:0] d;
    output reg signed [8:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 9'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$sum1


module top$sum1_1(d,q,clock);
    
    // Module arguments
    input wire signed [9:0] d;
    output reg signed [9:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 10'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$sum1_1


module top$sum1_2(d,q,clock);
    
    // Module arguments
    input wire signed [9:0] d;
    output reg signed [9:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 10'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$sum1_2


module top$sum2(d,q,clock);
    
    // Module arguments
    input wire signed [8:0] d;
    output reg signed [8:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 9'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$sum2


module top$sum3(d,q,clock);
    
    // Module arguments
    input wire signed [8:0] d;
    output reg signed [8:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 9'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$sum3


module top$sum4(d,q,clock);
    
    // Module arguments
    input wire signed [8:0] d;
    output reg signed [8:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 9'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$sum4
