`timescale 1ps / 1ps

module tb_reconocedorPatrones;

    // Señales
    reg clk;
    reg rst;
    reg in;
    wire out;

    // Instancia del módulo bajo prueba (UUT - Unit Under Test)
    reconocedor_patrones uut (
        .clk(clk),
        .rst(rst),
        .in(in),
        .out(out)
    );

    // Generador de reloj (período de 10 ns)
    always #1 clk = ~clk;

    task apply_input;
        input val;
        begin
            in = val;
            #2;
            $display("t=%0t | in=%b | out=%b", $time, in, out);
        end
    endtask

    // Proceso de prueba
    initial begin
        $dumpfile("reconocedorPatronesVerilogWave.vcd");
	    $dumpvars(0, tb_reconocedorPatrones);

        // Inicialización
        clk = 0;
        rst = 1;
        in = 0;
        #2; // Esperar 10 ns

        rst = 0; // Liberar el reset
        #2;

        $display("Inicio de test del reconocedor de patrones (buscando 1011):");
        
        // Primera prueba: 1011 (patrón correcto)
        apply_input(1); //S1
        apply_input(0); //S2
        apply_input(1); //S3
        apply_input(1); //S4 -> salida = 1
       

        // Segunda prueba: patrón parcial (101)
        apply_input(1); 
        apply_input(0); 
        apply_input(1); 

        //Tercera prueba: ruido + patrón correcto + superposicion
        apply_input(0); //Ruido
        apply_input(1); 
        apply_input(0); 
        apply_input(1); //superposicion
        apply_input(0); 
        apply_input(1); 
        apply_input(1); //salida -> 1

        $display("Test finalizado");
        $finish; // Finaliza la simulación
    end

endmodule
