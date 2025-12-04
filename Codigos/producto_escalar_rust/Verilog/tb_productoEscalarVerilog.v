`timescale 1ps / 1ps

module tb_productoEscalar;
    reg clk;
    reg reset;
    reg start;
    reg [7:0] a, b;
    reg valid;
    wire [15:0] result;
    wire busy;

    // Instanciamos el módulo bajo prueba
    productoEscalar uut (
        .clk(clk),
        .reset(reset),
        .start(start),
        .a(a),
        .b(b),
        .valid(valid),
        .result(result),
        .busy(busy)
    );

    // Generamos el reloj
    always begin
        #1 clk = ~clk; // Reloj con periodo de 10 unidades de tiempo
    end

    // Monitor para mostrar señales por consola
    initial begin
        $display("Tiempo | clk | reset | start | valid |   a   |   b   | busy |  result");
        $monitor("%5t   |  %b  |   %b   |   %b   |   %b   | %3d  | %3d  |  %b   | %5d", 
                  $time, clk, reset, start, valid, a, b, busy, result);
    end

    // Inicialización y prueba
    initial begin
        $dumpfile("productoEscalarVerilogWave.vcd");
	    $dumpvars(0, tb_productoEscalar);

        // Inicializamos las señales
        clk = 0;
        reset = 1;
        start = 0;
        valid = 0;
        a = 8'b0;
        b = 8'b0;

        // Esperamos un ciclo de reloj
        #2;
        reset = 0;

        // Iniciamos el cálculo
        start = 1;
        #2;
        
        start = 0;
        valid = 1;
        // Proporcionamos datos de entrada

        a = 8'd3;  // A[0] = 3
        b = 8'd4;  // B[0] = 4
        #2;
        a = 8'd5;  // A[1] = 5
        b = 8'd6;  // B[1] = 6
        #2;
        a = 8'd7;  // A[2] = 7
        b = 8'd8;  // B[2] = 8
        #2;
        a = 8'd9;  // A[3] = 9
        b = 8'd10; // B[3] = 10
        #2;

        // Mostramos el resultado
        $display("Resultado: %d", result);
        $finish;
    end
endmodule
