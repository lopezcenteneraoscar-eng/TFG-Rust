module reconocedor_patrones (
    input wire clk,
    input wire rst,
    input wire in,
    output reg out
);


    parameter S0 = 3'b000; // Estado inicial
    parameter S1 = 3'b001; // Detecta '1'
    parameter S2 = 3'b010; // Detecta '10'
    parameter S3 = 3'b011; // Detecta '101'
    parameter S4 = 3'b100; // Detecta '1011'
   

    reg[2:0] state, next_state;

    always @(posedge clk or posedge rst) begin
        if (rst)
            state <= S0;
        else
            state <= next_state;
    end

    always @(*) begin
        out = 1'b0; // Salida por defecto
        case (state)
            S0: next_state = in ? S1 : S0;
            S1: next_state = in ? S1 : S2;
            S2: next_state = in ? S3 : S0;
            S3: next_state = in ? S4 : S2;
            S4: begin
                out = 1'b1;
                next_state = S1;
            end
            default: next_state = S0;
        endcase
    end

endmodule
