`timescale 1ps/1ps

module tb_multiplicacionMatrices;

    // Entradas
    reg signed [3:0] a0, a1, a2, a3;
    reg signed [3:0] b0, b1, b2, b3;

    // Salidas
    wire signed [31:0] result0, result1, result2, result3;

    // Instancia del módulo
    multiplicacion_Matrices uut (
        .a00(a0), .a01(a1), .a10(a2), .a11(a3),
        .b00(b0), .b01(b1), .b10(b2), .b11(b3),
        .c00(result0), .c01(result1),
        .c10(result2), .c11(result3)
    );

    integer i, j, k, l, m, n, o, p;

    initial begin
        $dumpfile("multiplicacionMatricesVerilogWave.vcd");
	    $dumpvars(0, tb_multiplicacionMatrices);
        a0=0;a1=0;a2=0;a3=0;
        b0=0;b1=0;b2=0;b3=0;
        #1;

        for (i = -2; i <= 2; i = i + 1)
        for (j = -2; j <= 2; j = j + 1)
        for (k = -2; k <= 2; k = k + 1)
        for (l = -2; l <= 2; l = l + 1)
        for (m = -2; m <= 2; m = m + 1)
        for (n = -2; n <= 2; n = n + 1)
        for (o = -2; o <= 2; o = o + 1)
        for (p = -2; p <= 2; p = p + 1) begin

            a0 = i; a1 = j; a2 = k; a3 = l;
            b0 = m; b1 = n; b2 = o; b3 = p;
            #1;

        end
        $display("==FIN TEST===");
        $finish;
    end

endmodule
