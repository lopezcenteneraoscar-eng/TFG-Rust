module productoEscalar (
    input clk,              // Reloj
    input reset,            // Reset asincrónico
    input start,            // Señal de inicio del cálculo
    input [7:0] a,          // Entrada A (elemento del vector)
    input [7:0] b,          // Entrada B (elemento del vector)
    input valid,            // Señal de validez de los datos
    output reg [15:0] result, // Resultado acumulado
    output reg busy         // Indica si está ocupado
);

    reg [15:0] accumulator;  // Acumulador del resultado
    reg [3:0] counter;       // Contador de ciclos
    reg [3:0] max_count;     // Tamaño del vector - 1

    wire [15:0] producto;

    // Instanciamos el multiplicador tipo RustHDL
    mul8x8 mul_inst (
        .a(a),
        .b(b),
        .resul(producto)
    );

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            accumulator <= 0;
            counter <= 0;
            result <= 0;
            busy <= 0;
            max_count <= 4;
        end else if (start && !busy) begin
            accumulator <= 0;
            counter <= 0;
            result <= 0;
            busy <= 1;
        end else if (busy) begin
            if (valid) begin
                accumulator <= accumulator + producto;
                counter <= counter + 1;
                if (counter == max_count) begin
                    result <= accumulator + producto;
                    busy <= 0;
                end
            end
        end
    end

    always @(*) begin
        if (counter == max_count) begin
            result <= accumulator;
            busy <= 0;
        end
end
endmodule

// Multiplicador estilo RustHDL (shift-and-add)
module mul8x8 (
    input [7:0] a,
    input [7:0] b,
    output reg [15:0] resul
);
    always @(*) begin
        resul = 16'd0;
        if (b[0]) resul = resul + (a << 0);
        if (b[1]) resul = resul + (a << 1);
        if (b[2]) resul = resul + (a << 2);
        if (b[3]) resul = resul + (a << 3);
        if (b[4]) resul = resul + (a << 4);
        if (b[5]) resul = resul + (a << 5);
        if (b[6]) resul = resul + (a << 6);
        if (b[7]) resul = resul + (a << 7);
    end
endmodule
