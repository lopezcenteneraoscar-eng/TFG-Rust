use rust_hdl::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

// Definición del módulo Reducción en Árbol
#[derive(LogicBlock, Clone)]
struct ArbolSumadoresSegmentacion {
    pub inputs: [Signal<In, Signed<8>>; 8],  // Entradas de 8 elementos
    pub result: Signal<Out, Signed<11>>,     // Resultado de la reducción
    pub clk: Signal<In, Clock>,
    pub rstn: Signal<In, Bit>,

    // Etapa 1
    sum1: DFF<Signed<9>>,
    sum2: DFF<Signed<9>>,
    sum3: DFF<Signed<9>>,
    sum4: DFF<Signed<9>>,

    // Etapa 2
    sum1_1: DFF<Signed<10>>,
    sum1_2: DFF<Signed<10>>,

    // Etapa 3
    final_sum: DFF<Signed<11>>,
}

impl Default for ArbolSumadoresSegmentacion {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            result: Default::default(),
            clk: Default::default(),
            rstn: Default::default(),
            sum1: DFF::default(),
            sum2: DFF::default(),
            sum3: DFF::default(),
            sum4: DFF::default(),
            sum1_1: DFF::default(),
            sum1_2: DFF::default(),
            final_sum: DFF::default(),
        }
    }
}

impl Logic for ArbolSumadoresSegmentacion {
    #[hdl_gen]
    fn update(&mut self) {
        self.sum1.clock.next = self.clk.val();
        self.sum2.clock.next = self.clk.val();
        self.sum3.clock.next = self.clk.val();
        self.sum4.clock.next = self.clk.val();
        self.sum1_1.clock.next = self.clk.val();
        self.sum1_2.clock.next = self.clk.val();
        self.final_sum.clock.next = self.clk.val();

        if !self.rstn.val() {
            self.sum1.d.next = 0.into();
            self.sum2.d.next = 0.into();
            self.sum3.d.next = 0.into();
            self.sum4.d.next = 0.into();
            self.sum1_1.d.next = 0.into();
            self.sum1_2.d.next = 0.into();
            self.final_sum.d.next = 0.into();
            self.result.next = 0.into();
        } else {
            // Etapa 1: Sumar entradas en pares
            self.sum1.d.next = signed_bit_cast::<9,8>(self.inputs[0].val()) + signed_bit_cast::<9,8>(self.inputs[1].val());
            self.sum2.d.next = signed_bit_cast::<9,8>(self.inputs[2].val()) + signed_bit_cast::<9,8>(self.inputs[3].val());
            self.sum3.d.next = signed_bit_cast::<9,8>(self.inputs[4].val()) + signed_bit_cast::<9,8>(self.inputs[5].val());
            self.sum4.d.next = signed_bit_cast::<9,8>(self.inputs[6].val()) + signed_bit_cast::<9,8>(self.inputs[7].val());

            // Etapa 2: Sumar resultados intermedios
            self.sum1_1.d.next = signed_bit_cast::<10,9>(self.sum1.q.val()) + signed_bit_cast::<10,9>(self.sum2.q.val());
            self.sum1_2.d.next = signed_bit_cast::<10,9>(self.sum3.q.val()) + signed_bit_cast::<10,9>(self.sum4.q.val());

            // Etapa 3: Sumar el resultado final
            self.final_sum.d.next = signed_bit_cast::<11,10>(self.sum1_1.q.val()) + signed_bit_cast::<11,10>(self.sum1_2.q.val());

            self.result.next = self.final_sum.q.val();
        }
    }
}

fn main() {
    let mut uut = ArbolSumadoresSegmentacion::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module ArbolSumadoresSegmentacion(");
    let file_path = "ArbolSumadoresSegmentacionRust.v";

    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}

#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    fs::write("test_ArbolSumadoresSegmentacion.v", tb)?;
    let compile = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_ArbolSumadoresSegmentacion.v"])
        .output()?;

    println!("Compilación iVerilog: {:?}", compile);

    let output = std::process::Command::new("vvp")
        .arg("test_tb.vvp")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into())
}

#[test]
fn test_reduccion_arbol() -> anyhow::Result<()> {
    let mut uut = ArbolSumadoresSegmentacion::default();
    uut.connect_all();

    let verilog_tb = r#"
module test;

    reg signed [7:0] in0, in1, in2, in3, in4, in5, in6, in7;
    wire signed [10:0] result;
    reg clk;
    reg rstn;

    ArbolSumadoresSegmentacion uut(
        .inputs$0(in0), .inputs$1(in1), .inputs$2(in2), .inputs$3(in3),
        .inputs$4(in4), .inputs$5(in5), .inputs$6(in6), .inputs$7(in7),
        .result(result), .clk(clk), .rstn(rstn)
    );

    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end

    integer cycle;

    initial begin
        
        //Reset
        rstn = 0;
        {in0,in1,in2,in3,in4,in5,in6,in7} = 0;
        #12;
        rstn = 1;

        $display("Empieza el test");
        for (cycle = 0; cycle < 6; cycle = cycle + 1) begin
            case (cycle)
                0: begin
                    $display("Empieza a sumar el caso 0, resultado en 3 ciclos de 12");
                    in0 = 0; in1 = -1; in2 = 2; in3 = 3;
                    in4 = 4; in5 = 5; in6 = 6; in7 = -7;
                end
                1: begin
                    $display("Empieza a sumar el caso 2, resultado en 3 ciclos de -1024");
                    in0 = -128; in1 = -128; in2 = -128; in3 = -128;
                    in4 = -128; in5 = -128; in6 = -128; in7 = -128;
                end
                2: begin
                    $display("Empieza a sumar el caso 2, resultado en 3 ciclos de 125");
                    in0 = 10; in1 = 15; in2 = 20; in3 = 5;
                    in4 = 30; in5 = 25; in6 = 12; in7 = 8;
                end
                3: begin
                    $display("Empieza a sumar el caso 3, resultado en 3 ciclos de 1016");
                    in0 = 127; in1 = 127; in2 = 127; in3 = 127;
                    in4 = 127; in5 = 127; in6 = 127; in7 = 127;
                end
                default: begin
                    $display("Caso por defecto, suma todo 0");
                    {in0,in1,in2,in3,in4,in5,in6,in7} = 0;
                end
            endcase

            #10;
            $display("Cycle %0d: Result: %0d", cycle, result);
        end

        $finish;
    end
endmodule
"#;

    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    let code = tb.replace("module top(", "module ArbolSumadoresSegmentacion(");
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("Salida Verilog:\n{}", sim_output);

    let mut sim = Simulation::<ArbolSumadoresSegmentacion>::new();
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;

        // Casos de prueba: (entradas, suma esperada, nombre del caso)
        let test_cases: Vec<([i8; 8], i16, &'static str)> = vec![
            ([0, -1, 2, 3, 4, 5, 6, -7], 12, "caso 0"),         
            ([-127, -127, -127, -127, -127, -127, -127, -127], -1016, "caso 1"),         
            ([10, 15, 20, 5, 30, 25, 12, 8], 125, "caso 2"),  
            ([127, 127, 127, 127, 127, 127, 127, 127], 1016, "caso 3"), 
            ([0, 0, 0, 0, 0, 0, 0, 0], 0, "default"),  
            ([0, 0, 0, 0, 0, 0, 0, 0], 0, "default"),         
        ];

        println!("Empieza el test");

        x.rstn.next = false;
        x.clk.next = Clock { clk: false };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.clk.next = Clock { clk: true };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.rstn.next = true;
        x.clk.next = Clock { clk: false };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.clk.next = Clock { clk: true };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        for (cycle, (inputs, expected, case_name)) in test_cases.iter().enumerate() {
            println!("Empieza a sumar el {}, resultado en 3 ciclos de {}", case_name, expected);

            // Cargar inputs
            for i in 0..8 {
                x.inputs[i].next = Signed::<8>::from(inputs[i] as i64);
            }

            x.clk.next = Clock { clk: false };
            let x_clone = x.clone();
            x = ep.wait(1, x_clone)?;

            x.clk.next = Clock { clk: true };
            let x_clone = x.clone();
            x = ep.wait(1, x_clone)?;

            println!("Cycle {}: Result: {}", cycle, x.result.val().bigint());
        }

        ep.done(x)?;
        Ok(())
    });

    let _ = fs::remove_file("test_tb.vvp");
    let _ = fs::remove_file("test_ArbolSumadoresSegmentacion.v");

    sim.run_to_file(Box::new(uut), 100_000, "ArbolSumadoresSegmentacionWave.vcd")
        .map_err(|e| anyhow!("{:?}", e))?;

    Ok(())
}