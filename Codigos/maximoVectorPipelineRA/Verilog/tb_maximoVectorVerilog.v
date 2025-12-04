`timescale 1ps / 1ps

module tb_maximoVectorSegmentacion;

    // Declarar señales
    reg clk;
    reg rst;
    reg valid_in;
    reg signed [7:0] data_in_1;
    reg signed [7:0] data_in_2;
    reg signed [7:0] data_in_3;
    reg signed [7:0] data_in_4;
    reg signed [7:0] data_in_5;
    reg signed [7:0] data_in_6;
    reg signed [7:0] data_in_7;
    reg signed [7:0] data_in_8;
    wire signed [7:0] max_out;
    wire valid_out;

    integer cycle;

    // Instanciación del módulo a testear
    max_pipeline_tree uut (
        .clk(clk),
        .rst(rst),
        .valid_in(valid_in),
        .data_in_1(data_in_1),
        .data_in_2(data_in_2),
        .data_in_3(data_in_3),
        .data_in_4(data_in_4),
        .data_in_5(data_in_5),
        .data_in_6(data_in_6),
        .data_in_7(data_in_7),
        .data_in_8(data_in_8),
        .max_out(max_out),
        .valid_out(valid_out)
    );

    // Inicialización de señales
    initial begin
        $dumpfile("maximoVectorSegmentacionVerilogWave.vcd");
        $dumpvars(0, tb_maximoVectorSegmentacion);

        clk = 0;
        rst = 0;
        valid_in = 0;
        data_in_1 = 0;
        data_in_2 = 0;
        data_in_3 = 0;
        data_in_4 = 0;
        data_in_5 = 0;
        data_in_6 = 0;
        data_in_7 = 0;
        data_in_8 = 0;

        // Resetear el sistema
        rst = 1;
        #2 rst = 0;
        #2 valid_in = 1;  // Habilitar la entrada

        // Caso 1: Solo valores negativos
        // Vector: [-50, -20, -100, -5, -30, -90, -10, -60]
        data_in_1 = -50;
        data_in_2 = -20;
        data_in_3 = -100;
        data_in_4 = -5;
        data_in_5 = -30;
        data_in_6 = -90;
        data_in_7 = -10;
        data_in_8 = -60;

        #2;  // Esperar un ciclo de reloj

        // Verificar la salida
        $display("Ciclo 2 - Vector: [-50, -20, -100, -5, -30, -90, -10, -60]");
        $display("Max Output: %d, Valid Output: %b", max_out, valid_out);
        
        // Caso 2: Probar con valores negativos, cero y positivos
        // Vector: [-128, 0, 50, 120, -100, 50, 120, 127]
        data_in_1 = -128;  // Valor negativo mínimo
        data_in_2 = 0;     // Cero
        data_in_3 = 50;    // Valor positivo
        data_in_4 = 120;   // Valor positivo
        data_in_5 = -100;  // Valor negativo
        data_in_6 = 50;    // Valor positivo
        data_in_7 = 120;   // Valor positivo
        data_in_8 = 127;   // Valor positivo máximo

        #2;  // Esperar un ciclo de reloj

        // Verificar la salida
        $display("Ciclo 1 - Vector: [-128, 0, 50, 120, -100, 50, 120, 127]");
        $display("Max Output: %d, Valid Output: %b", max_out, valid_out);

        // Caso 3: Solo valores positivos
        // Vector: [30, 50, -80, 120, 0, 60, 70, 110]
        data_in_1 = 30;
        data_in_2 = 50;
        data_in_3 = -80;
        data_in_4 = 120;
        data_in_5 = 0;
        data_in_6 = 60;
        data_in_7 = 70;
        data_in_8 = 110;
       
        // Verificar la salida
        $display("Ciclo 3 - Vector: [30, 50, -80, 120, 0, 60, 70, 110]");
        $display("Max Output: %d, Valid Output: %b", max_out, valid_out);

        for (cycle = 4; cycle < 7; cycle = cycle + 1) begin
            #2;
            $display("Ciclo %d", cycle);
            $display("Max Output: %d, Valid Output: %b", max_out, valid_out);
        end
        $finish;
    end

    // Generación de reloj
    always #1 clk = ~clk;

endmodule