

module productoEscalar(clock,reset,start,a,b,valid,result,busy);
    
    // Module arguments
    input wire  clock;
    input wire  reset;
    input wire  start;
    input wire  [7:0] a;
    input wire  [7:0] b;
    input wire  valid;
    output reg  [15:0] result;
    output reg  busy;
    
    // Stub signals
    reg  [15:0] accumulator$d;
    wire  [15:0] accumulator$q;
    reg  accumulator$clk;
    reg  [7:0] counter$d;
    wire  [7:0] counter$q;
    reg  counter$clk;
    reg  e_busy$d;
    wire  e_busy$q;
    reg  e_busy$clk;
    reg  [7:0] mul$a;
    reg  [7:0] mul$b;
    wire  [15:0] mul$resul;
    
    // Local signals
    reg  [7:0] max_count;
    
    // Sub module instances
    top$accumulator accumulator(
        .d(accumulator$d),
        .q(accumulator$q),
        .clk(accumulator$clk)
    );
    top$counter counter(
        .d(counter$d),
        .q(counter$q),
        .clk(counter$clk)
    );
    top$e_busy e_busy(
        .d(e_busy$d),
        .q(e_busy$q),
        .clk(e_busy$clk)
    );
    top$mul mul(
        .a(mul$a),
        .b(mul$b),
        .resul(mul$resul)
    );
    
    // Update code
    always @(*) begin
        accumulator$clk = clock;
        counter$clk = clock;
        e_busy$clk = clock;
        accumulator$d = accumulator$q;
        counter$d = counter$q;
        e_busy$d = e_busy$q;
        max_count = 32'h4;
        result = 32'h0;
        if (reset) begin
            accumulator$d = 32'h0;
            counter$d = 32'h0;
            e_busy$d = 1'b0;
            busy = 1'b0;
        end
        else if (~e_busy$q) begin
            if (start) begin
                e_busy$d = 1'b1;
                counter$d = 32'h0;
                accumulator$d = 32'h0;
                busy = 1'b1;
            end
        end
        else begin
            if (valid) begin
                mul$a = a;
                mul$b = b;
                accumulator$d = accumulator$q + mul$resul;
                counter$d = counter$q + 32'h1;
                if (counter$q == max_count) begin
                    result = accumulator$q;
                    e_busy$d = 1'b0;
                    busy = 1'b0;
                end
            end
        end
    end
    
endmodule // top


module top$accumulator(d,q,clk);
    
    // Module arguments
    input wire  [15:0] d;
    output reg  [15:0] q;
    input wire  clk;
    
    // Update code (custom)
    initial begin
       q = 16'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$accumulator


module top$counter(d,q,clk);
    
    // Module arguments
    input wire  [7:0] d;
    output reg  [7:0] q;
    input wire  clk;
    
    // Update code (custom)
    initial begin
       q = 8'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$counter


module top$e_busy(d,q,clk);
    
    // Module arguments
    input wire  d;
    output reg  q;
    input wire  clk;
    
    // Update code (custom)
    initial begin
       q = 1'h0;
    end
    
    always @(posedge clk) q <= d;
endmodule // top$e_busy


module top$mul(a,b,resul);
    
    // Module arguments
    input wire  [7:0] a;
    input wire  [7:0] b;
    output reg  [15:0] resul;
    
    // Update code
    always @(*) begin
        resul = 32'h0;
        if (b[32'h0]) begin
            resul = resul + ((a) & 16'hffff);
        end
        if (b[32'h1]) begin
            resul = resul + (((a) & 16'hffff) << 32'h1);
        end
        if (b[32'h2]) begin
            resul = resul + (((a) & 16'hffff) << 32'h2);
        end
        if (b[32'h3]) begin
            resul = resul + (((a) & 16'hffff) << 32'h3);
        end
        if (b[32'h4]) begin
            resul = resul + (((a) & 16'hffff) << 32'h4);
        end
        if (b[32'h5]) begin
            resul = resul + (((a) & 16'hffff) << 32'h5);
        end
        if (b[32'h6]) begin
            resul = resul + (((a) & 16'hffff) << 32'h6);
        end
        if (b[32'h7]) begin
            resul = resul + (((a) & 16'hffff) << 32'h7);
        end
    end
    
endmodule // top$mul
