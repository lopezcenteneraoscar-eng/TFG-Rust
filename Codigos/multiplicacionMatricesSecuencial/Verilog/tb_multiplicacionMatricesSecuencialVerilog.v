`timescale 1ps/1ps

module tb_multiplicacionMatricesSecuencial();

reg clk;
reg rst;
reg start;
reg signed [3:0] a11, a12, a21, a22;
reg signed [3:0] b11, b12, b21, b22;
wire done;
wire signed [8:0] c11, c12, c21, c22;

// Instancia del módulo
multiplicacionMatricesSecuencial uut (
    .clk(clk),
    .rst(rst),
    .start(start),
    .a11(a11), .a12(a12), .a21(a21), .a22(a22),
    .b11(b11), .b12(b12), .b21(b21), .b22(b22),
    .done(done),
    .c11(c11), .c12(c12),
    .c21(c21), .c22(c22)
);

// Generador de reloj
initial begin
    clk = 0;
    forever #1 clk = ~clk;
end

integer i1, i2, i3, i4, j1, j2, j3, j4;

initial begin
    $dumpfile("multiplicacionMatricesSecuencialVerilogWave.vcd");
    $dumpvars(0, tb_multiplicacionMatricesSecuencial);

    // Inicialización
    rst = 1;
    start = 0;
    a11 = 0; a12 = 0; a21 = 0; a22 = 0;
    b11 = 0; b12 = 0; b21 = 0; b22 = 0;
    #2;
    rst = 0;
    #2;

    // Barrido de valores entre -2 y 2
    for (i1 = -2; i1 <= 2; i1 = i1 + 1)
    for (i2 = -2; i2 <= 2; i2 = i2 + 1)
    for (i3 = -2; i3 <= 2; i3 = i3 + 1)
    for (i4 = -2; i4 <= 2; i4 = i4 + 1)
    for (j1 = -2; j1 <= 2; j1 = j1 + 1)
    for (j2 = -2; j2 <= 2; j2 = j2 + 1)
    for (j3 = -2; j3 <= 2; j3 = j3 + 1)
    for (j4 = -2; j4 <= 2; j4 = j4 + 1)
    begin
        a11 = i1; a12 = i2; a21 = i3; a22 = i4;
        b11 = j1; b12 = j2; b21 = j3; b22 = j4;

        start = 1;
        #2 start = 0;

        wait(done);
        #2;

    end
    $finish;
end

endmodule