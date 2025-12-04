`timescale 1ps / 1ps

module tb_ArbolSumadoresSegmentacion;

	parameter WIDTH = 8;

	reg clk;
	reg rstn;
	reg signed [WIDTH-1:0] in0, in1, in2, in3, in4, in5, in6, in7;
	wire signed [WIDTH+2:0] result;

	ArbolSumadoresSegmentacion #(.WIDTH(WIDTH)) dut (
		.clk(clk),
		.rstn(rstn),
		.in0(in0), .in1(in1), .in2(in2), .in3(in3), 
		.in4(in4), .in5(in5), .in6(in6), .in7(in7),
		.result(result)
	);

	//Reloj de 1ps
	initial begin
		clk = 0;
		forever #1 clk = ~clk;
	end

	integer cycle;

	initial begin
        $dumpfile("ArbolSumadoresSegmentacionVerilogWave.vcd");
        $dumpvars(0, tb_ArbolSumadoresSegmentacion);

        // Reset
        rstn = 0;
        in0 = 0; in1 = 0; in2 = 0; in3 = 0;
        in4 = 0; in5 = 0; in6 = 0; in7 = 0;
        
        #2;
        rstn = 1;
        #2;

        $display("Empieza el test");
        // Test de segmentación: 4 sets de datos, uno por ciclo
        for (cycle = 0; cycle < 6; cycle = cycle + 1) begin
            case (cycle)
                0: begin
                    $display("Empieza a sumar el caso 1, resultado en 3 ciclos de 12");
                    in0 = 0; in1 = -1; in2 = 2; in3 = 3;
                    in4 = 4; in5 = 5; in6 = 6; in7 = -7; // 12
                end
                1: begin
                    $display("Empieza a sumar el caso 2, resultado en 3 ciclos de -1024");
                    in0 = -128; in1 = -128; in2 = -128; in3 = -128;
                    in4 = -128; in5 = -128; in6 = -128; in7 = -128; // -1024
                end
                2: begin
					$display("Empieza a sumar el caso 3, resultado en 3 ciclos de 125");
                    in0 = 10; in1 = 15; in2 = 20; in3 = 5;
                    in4 = 30; in5 = 25; in6 = 12; in7 = 8; // 125
                end
                3: begin
					$display("Empieza a sumar el caso 4, resultado en 3 ciclos de 1016");
                    in0 = 127; in1 = 127; in2 = 127; in3 = 127;
                    in4 = 127; in5 = 127; in6 = 127; in7 = 127; // 1016
                end
                default: begin
					$display("Caso por defecto, suma todo 0");
                    in0 = 0; in1 = 0; in2 = 0; in3 = 0;
                    in4 = 0; in5 = 0; in6 = 0; in7 = 0;
                end
            endcase

            #2;
            $display("Cycle %0d -> Resultado: %0d", cycle, result);
        end

        $finish;
    end
	
endmodule 		
