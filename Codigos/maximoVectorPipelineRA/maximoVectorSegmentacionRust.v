

module maximoVectorSegmentacion(inputs$0,inputs$1,inputs$2,inputs$3,inputs$4,inputs$5,inputs$6,inputs$7,result,clk,rst,valid_in,valid_out);
    
    // Module arguments
    input wire signed [7:0] inputs$0;
    input wire signed [7:0] inputs$1;
    input wire signed [7:0] inputs$2;
    input wire signed [7:0] inputs$3;
    input wire signed [7:0] inputs$4;
    input wire signed [7:0] inputs$5;
    input wire signed [7:0] inputs$6;
    input wire signed [7:0] inputs$7;
    output reg signed [7:0] result;
    input wire  clk;
    input wire  rst;
    input wire  valid_in;
    output reg  valid_out;
    
    // Stub signals
    reg signed [7:0] max1$d;
    wire signed [7:0] max1$q;
    reg  max1$clock;
    reg signed [7:0] max2$d;
    wire signed [7:0] max2$q;
    reg  max2$clock;
    reg signed [7:0] max3$d;
    wire signed [7:0] max3$q;
    reg  max3$clock;
    reg signed [7:0] max4$d;
    wire signed [7:0] max4$q;
    reg  max4$clock;
    reg  valid_stage1$d;
    wire  valid_stage1$q;
    reg  valid_stage1$clock;
    reg signed [7:0] max1_1$d;
    wire signed [7:0] max1_1$q;
    reg  max1_1$clock;
    reg signed [7:0] max1_2$d;
    wire signed [7:0] max1_2$q;
    reg  max1_2$clock;
    reg  valid_stage2$d;
    wire  valid_stage2$q;
    reg  valid_stage2$clock;
    reg signed [7:0] final_max$d;
    wire signed [7:0] final_max$q;
    reg  final_max$clock;
    reg  valid_stage3$d;
    wire  valid_stage3$q;
    reg  valid_stage3$clock;
    
    // Sub module instances
    top$max1 max1(
        .d(max1$d),
        .q(max1$q),
        .clock(max1$clock)
    );
    top$max2 max2(
        .d(max2$d),
        .q(max2$q),
        .clock(max2$clock)
    );
    top$max3 max3(
        .d(max3$d),
        .q(max3$q),
        .clock(max3$clock)
    );
    top$max4 max4(
        .d(max4$d),
        .q(max4$q),
        .clock(max4$clock)
    );
    top$valid_stage1 valid_stage1(
        .d(valid_stage1$d),
        .q(valid_stage1$q),
        .clock(valid_stage1$clock)
    );
    top$max1_1 max1_1(
        .d(max1_1$d),
        .q(max1_1$q),
        .clock(max1_1$clock)
    );
    top$max1_2 max1_2(
        .d(max1_2$d),
        .q(max1_2$q),
        .clock(max1_2$clock)
    );
    top$valid_stage2 valid_stage2(
        .d(valid_stage2$d),
        .q(valid_stage2$q),
        .clock(valid_stage2$clock)
    );
    top$final_max final_max(
        .d(final_max$d),
        .q(final_max$q),
        .clock(final_max$clock)
    );
    top$valid_stage3 valid_stage3(
        .d(valid_stage3$d),
        .q(valid_stage3$q),
        .clock(valid_stage3$clock)
    );
    
    // Update code
    always @(*) begin
        max1$clock = clk;
        max2$clock = clk;
        max3$clock = clk;
        max4$clock = clk;
        max1_1$clock = clk;
        max1_2$clock = clk;
        final_max$clock = clk;
        valid_stage1$clock = clk;
        valid_stage2$clock = clk;
        valid_stage3$clock = clk;
        if (rst) begin
            max1$d = 32'h0;
            max2$d = 32'h0;
            max3$d = 32'h0;
            max4$d = 32'h0;
            max1_1$d = 32'h0;
            max1_2$d = 32'h0;
            final_max$d = 32'h0;
            valid_stage1$d = 1'b0;
            valid_stage2$d = 1'b0;
            valid_stage3$d = 1'b0;
            result = 32'h0;
            valid_out = 1'b0;
        end
        else begin
            if (valid_in) begin
                if (inputs$0 > inputs$1) begin
                    max1$d = inputs$0;
                end
                else begin
                    max1$d = inputs$1;
                end
                if (inputs$2 > inputs$3) begin
                    max2$d = inputs$2;
                end
                else begin
                    max2$d = inputs$3;
                end
                if (inputs$4 > inputs$5) begin
                    max3$d = inputs$4;
                end
                else begin
                    max3$d = inputs$5;
                end
                if (inputs$6 > inputs$7) begin
                    max4$d = inputs$6;
                end
                else begin
                    max4$d = inputs$7;
                end
            end
            else begin
                max1$d = 32'h0;
                max2$d = 32'h0;
                max3$d = 32'h0;
                max4$d = 32'h0;
            end
            valid_stage1$d = valid_in;
            if (max1$q > max2$q) begin
                max1_1$d = max1$q;
            end
            else begin
                max1_1$d = max2$q;
            end
            if (max3$q > max4$q) begin
                max1_2$d = max3$q;
            end
            else begin
                max1_2$d = max4$q;
            end
            valid_stage2$d = valid_stage1$q;
            if (max1_1$q > max1_2$q) begin
                final_max$d = max1_1$q;
            end
            else begin
                final_max$d = max1_2$q;
            end
            valid_stage3$d = valid_stage2$q;
            result = final_max$q;
            valid_out = valid_stage3$q;
        end
    end
    
endmodule // top


module top$final_max(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$final_max


module top$max1(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$max1


module top$max1_1(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$max1_1


module top$max1_2(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$max1_2


module top$max2(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$max2


module top$max3(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$max3


module top$max4(d,q,clock);
    
    // Module arguments
    input wire signed [7:0] d;
    output reg signed [7:0] q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$max4


module top$valid_stage1(d,q,clock);
    
    // Module arguments
    input wire  d;
    output reg  q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 1'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$valid_stage1


module top$valid_stage2(d,q,clock);
    
    // Module arguments
    input wire  d;
    output reg  q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 1'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$valid_stage2


module top$valid_stage3(d,q,clock);
    
    // Module arguments
    input wire  d;
    output reg  q;
    input wire  clock;
    
    // Update code (custom)
    initial begin
       q = 1'h0;
    end
    
    always @(posedge clock) begin
       q <= d;
    end
          
endmodule // top$valid_stage3
