

module reconocedorPatrones(clk,rst,entrada,salida);
    
    // Module arguments
    input wire  clk;
    input wire  rst;
    input wire  entrada;
    output reg  salida;
    
    // Enums
    localparam State$S0 = 0;
    localparam State$S1 = 1;
    localparam State$S2 = 2;
    localparam State$S3 = 3;
    localparam State$S4 = 4;
    
    // Stub signals
    reg  [2:0] state$d;
    wire  [2:0] state$q;
    reg  state$clk;
    
    // Sub module instances
    top$state state(
        .d(state$d),
        .q(state$q),
        .clk(state$clk)
    );
    
    // Update code
    always @(*) begin
        state$clk = clk;
        salida = 1'b0;
        state$d = state$q;
        if (rst) begin
            state$d = State$S0;
        end
        else begin
            case (state$q)
                State$S0:
                    begin
                        if (entrada) begin
                            state$d = State$S1;
                        end
                    end
                State$S1:
                    begin
                        if (~entrada) begin
                            state$d = State$S2;
                        end
                    end
                State$S2:
                    begin
                        if (entrada) begin
                            state$d = State$S3;
                        end
                        else begin
                            state$d = State$S0;
                        end
                    end
                State$S3:
                    begin
                        if (entrada) begin
                            state$d = State$S4;
                        end
                        else begin
                            state$d = State$S2;
                        end
                    end
                State$S4:
                    begin
                        salida = 1'b1;
                        state$d = State$S0;
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
    localparam State$S0 = 0;
    localparam State$S1 = 1;
    localparam State$S2 = 2;
    localparam State$S3 = 3;
    localparam State$S4 = 4;
    
    // Update code (custom)
    initial begin
       q = 64'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$state
