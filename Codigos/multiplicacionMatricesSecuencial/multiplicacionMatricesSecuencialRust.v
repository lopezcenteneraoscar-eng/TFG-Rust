

module MultiplicacionMatricesSecuencial(clk,rst,start,a$0,a$1,a$2,a$3,b$0,b$1,b$2,b$3,result$0,result$1,result$2,result$3,done);
    
    // Module arguments
    input wire  clk;
    input wire  rst;
    input wire  start;
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
    output reg  done;
    
    // Enums
    localparam State$Idle = 0;
    localparam State$Calc1 = 1;
    localparam State$Calc2 = 2;
    localparam State$Calc3 = 3;
    localparam State$Calc4 = 4;
    localparam State$Done = 5;
    
    // Stub signals
    reg  [2:0] state$d;
    wire  [2:0] state$q;
    reg  state$clk;
    reg signed [31:0] temp1$d;
    wire signed [31:0] temp1$q;
    reg  temp1$clk;
    reg signed [31:0] temp2$d;
    wire signed [31:0] temp2$q;
    reg  temp2$clk;
    
    // Sub module instances
    top$state state(
        .d(state$d),
        .q(state$q),
        .clk(state$clk)
    );
    top$temp1 temp1(
        .d(temp1$d),
        .q(temp1$q),
        .clk(temp1$clk)
    );
    top$temp2 temp2(
        .d(temp2$d),
        .q(temp2$q),
        .clk(temp2$clk)
    );
    
    // Update code
    always @(*) begin
        state$clk = clk;
        temp1$clk = clk;
        temp2$clk = clk;
        done = 1'b0;
        if (rst) begin
            result$0 = 32'h0;
            result$1 = 32'h0;
            result$2 = 32'h0;
            result$3 = 32'h0;
            state$d = State$Idle;
            temp1$d = 32'h0;
            temp2$d = 32'h0;
        end
        else begin
            case (state$q)
                State$Idle:
                    begin
                        if (start) begin
                            state$d = State$Calc1;
                        end
                    end
                State$Calc1:
                    begin
                        temp1$d = $signed(a$0) * $signed(b$0);
                        temp2$d = $signed(a$1) * $signed(b$2);
                        state$d = State$Calc2;
                    end
                State$Calc2:
                    begin
                        result$0 = temp1$q + temp2$q;
                        temp1$d = $signed(a$0) * $signed(b$1);
                        temp2$d = $signed(a$1) * $signed(b$3);
                        state$d = State$Calc3;
                    end
                State$Calc3:
                    begin
                        result$1 = temp1$q + temp2$q;
                        temp1$d = $signed(a$2) * $signed(b$0);
                        temp2$d = $signed(a$3) * $signed(b$2);
                        state$d = State$Calc4;
                    end
                State$Calc4:
                    begin
                        result$2 = temp1$q + temp2$q;
                        temp1$d = $signed(a$2) * $signed(b$1);
                        temp2$d = $signed(a$3) * $signed(b$3);
                        state$d = State$Done;
                    end
                State$Done:
                    begin
                        result$3 = temp1$q + temp2$q;
                        done = 1'b1;
                        state$d = State$Idle;
                    end
            endcase
        end
    end
    
endmodule // top


module top$state(d,q,clk);
    
    // Module arguments
    input wire  [2:0] d;
    output reg  [2:0] q;
    input wire  clk;
    
    // Enums
    localparam State$Idle = 0;
    localparam State$Calc1 = 1;
    localparam State$Calc2 = 2;
    localparam State$Calc3 = 3;
    localparam State$Calc4 = 4;
    localparam State$Done = 5;
    
    // Update code (custom)
    initial begin
       q = 64'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$state


module top$temp1(d,q,clk);
    
    // Module arguments
    input wire signed [31:0] d;
    output reg signed [31:0] q;
    input wire  clk;
    
    // Update code (custom)
    initial begin
       q = 32'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$temp1


module top$temp2(d,q,clk);
    
    // Module arguments
    input wire signed [31:0] d;
    output reg signed [31:0] q;
    input wire  clk;
    
    // Update code (custom)
    initial begin
       q = 32'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$temp2
