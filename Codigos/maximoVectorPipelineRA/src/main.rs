use rust_hdl::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

// Definición del módulo Reducción en Árbol
#[derive(LogicBlock, Clone)]
struct maximoVectorSegmentacion {
    pub inputs: [Signal<In, Signed<8>>; 8],  // Entradas de 8 elementos
    pub result: Signal<Out, Signed<8>>,     // Resultado de la reducción
    pub clk: Signal<In, Clock>,
    pub rst: Signal<In, Bit>,
    pub valid_in: Signal<In, Bit>,
    pub valid_out: Signal<Out, Bit>,

    // Etapa 1
    max1: DFF<Signed<8>>,
    max2: DFF<Signed<8>>,
    max3: DFF<Signed<8>>,
    max4: DFF<Signed<8>>,
    valid_stage1: DFF<Bit>,

    // Etapa 2
    max1_1: DFF<Signed<8>>,
    max1_2: DFF<Signed<8>>,
    valid_stage2: DFF<Bit>,

    // Etapa 3
    final_max: DFF<Signed<8>>,
    valid_stage3: DFF<Bit>,
}

impl Default for maximoVectorSegmentacion {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            result: Default::default(),
            clk: Default::default(),
            rst: Default::default(),
            valid_in: Default::default(),
            valid_out: Default::default(),
            max1: DFF::default(),
            max2: DFF::default(),
            max3: DFF::default(),
            max4: DFF::default(),
            max1_1: DFF::default(),
            max1_2: DFF::default(),
            final_max: DFF::default(),
            valid_stage1: DFF::default(),
            valid_stage2: DFF::default(),
            valid_stage3: DFF::default(),
        }
    }
}

impl Logic for maximoVectorSegmentacion {
    #[hdl_gen]
    fn update(&mut self) {
        self.max1.clock.next = self.clk.val();
        self.max2.clock.next = self.clk.val();
        self.max3.clock.next = self.clk.val();
        self.max4.clock.next = self.clk.val();
        self.max1_1.clock.next = self.clk.val();
        self.max1_2.clock.next = self.clk.val();
        self.final_max.clock.next = self.clk.val();
        self.valid_stage1.clock.next = self.clk.val();
        self.valid_stage2.clock.next = self.clk.val();
        self.valid_stage3.clock.next = self.clk.val();

        if self.rst.val() {
            // Reset: limpiar todos los registros
            self.max1.d.next = 0.into();
            self.max2.d.next = 0.into();
            self.max3.d.next = 0.into();
            self.max4.d.next = 0.into();
            self.max1_1.d.next = 0.into();
            self.max1_2.d.next = 0.into();
            self.final_max.d.next = 0.into();

            self.valid_stage1.d.next = false;
            self.valid_stage2.d.next = false;
            self.valid_stage3.d.next = false;

            self.result.next = 0.into();
            self.valid_out.next = false;
        } else {
            // Solo si los datos de entrada son válidos, actualizamos la Etapa 1
            if self.valid_in.val() {
                if self.inputs[0].val() > self.inputs[1].val() {self.max1.d.next = self.inputs[0].val();}
                else {self.max1.d.next = self.inputs[1].val();}
                if self.inputs[2].val() > self.inputs[3].val() {self.max2.d.next = self.inputs[2].val();}
                else {self.max2.d.next = self.inputs[3].val();}
                if self.inputs[4].val() > self.inputs[5].val() {self.max3.d.next = self.inputs[4].val();}
                else {self.max3.d.next = self.inputs[5].val();}
                if self.inputs[6].val() > self.inputs[7].val() {self.max4.d.next = self.inputs[6].val();}
                else {self.max4.d.next = self.inputs[7].val();}
            }
            else {
                self.max1.d.next = 0.into();
                self.max2.d.next = 0.into();
                self.max3.d.next = 0.into();
                self.max4.d.next = 0.into();
            }
            self.valid_stage1.d.next = self.valid_in.val();

            // Etapa 2: Comparar registros
            if self.max1.q.val() > self.max2.q.val() {self.max1_1.d.next = self.max1.q.val();}
            else {self.max1_1.d.next = self.max2.q.val();}
            if self.max3.q.val() > self.max4.q.val() {self.max1_2.d.next = self.max3.q.val();}
            else {self.max1_2.d.next = self.max4.q.val();}
            self.valid_stage2.d.next = self.valid_stage1.q.val();

            // Etapa 3: Comparar los máximos intermedios
            if self.max1_1.q.val() > self.max1_2.q.val() {self.final_max.d.next = self.max1_1.q.val();}
            else {self.final_max.d.next = self.max1_2.q.val();}
            self.valid_stage3.d.next = self.valid_stage2.q.val();

            // Salida final
            self.result.next = self.final_max.q.val();
            self.valid_out.next = self.valid_stage3.q.val();
        }
    }
}

fn main() {
    let mut uut = maximoVectorSegmentacion::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module maximoVectorSegmentacion(");
    let file_path = "maximoVectorSegmentacionRust.v";

    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}

#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    fs::write("test_maximoVectorSegmentacion.v", tb)?;
    let compile = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_maximoVectorSegmentacion.v"])
        .output()?;

    println!("Compilación iVerilog: {:?}", compile);

    let output = std::process::Command::new("vvp")
        .arg("test_tb.vvp")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into())
}

#[test]
fn test_maximo_vector() -> anyhow::Result<()> {
    let mut uut = maximoVectorSegmentacion::default();
    uut.connect_all();

    let verilog_tb = r#"
module test;

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
    maximoVectorSegmentacion uut(
    .clk(clk), .rst(rst), .valid_in(valid_in),
        .inputs$0(data_in_1), .inputs$1(data_in_2), .inputs$2(data_in_3), .inputs$3(data_in_4),
        .inputs$4(data_in_5), .inputs$5(data_in_6), .inputs$6(data_in_7), .inputs$7(data_in_8),
        .result(max_out), .valid_out(valid_out)
    );

    //Reloj de 10ns
	initial begin
		clk = 0;
		forever #5 clk = ~clk;
	end

    // Inicialización de señales
    initial begin
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
        #10 rst = 0;
        #10 valid_in = 1;  // Habilitar la entrada

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

        #10;  // Esperar un ciclo de reloj

        // Verificar la salida
        $display("Ciclo 1 - Vector: [-50, -20, -100, -5, -30, -90, -10, -60]");
        $display("Max Output: %d, Valid Output: %b", max_out, 0);

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

        #10;  // Esperar un ciclo de reloj

        // Verificar la salida
        $display("Ciclo 2 - Vector: [-128, 0, 50, 120, -100, 50, 120, 127]");
        $display("Max Output: %d, Valid Output: %b", max_out, 0);
        
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

        #10;  // Esperar un ciclo de reloj
        valid_in = 0;
        // Verificar la salida
        $display("Ciclo 3 - Vector: [30, 50, -80, 120, 0, 60, 70, 110]");
        $display("Max Output: %d, Valid Output: %b", max_out, 0);

        for (cycle = 4; cycle < 7; cycle = cycle + 1) begin
            #10;
            $display("Ciclo %d", cycle);
            $display("Max Output: %d, Valid Output: %b", max_out, 0);
        end
        $finish;
    end
endmodule
"#;

    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    let code = tb.replace("module top(", "module maximoVectorSegmentacion(");
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("Salida Verilog:\n{}", sim_output);

    let mut sim = Simulation::<maximoVectorSegmentacion>::new();
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;

        // Casos de prueba: (entradas, suma esperada, nombre del caso)
        let test_cases: Vec<([i8; 8], i16, &'static str)> = vec![
            ([-50, -20, -100, -5, -30, -90, -10, -60], -5, "Ciclo 1 - Vector: [-50, -20, -100, -5, -30, -90, -10, -60]"),             
            ([-128, 0, 50, 120, -100, 50, 120, 127], 127, "Ciclo 2 - Vector: [-128, 0, 50, 120, -100, 50, 120, 127]"),                
            ([30, 50, -80, 120, 0, 60, 70, 110], 120, "Ciclo 3 - Vector: [30, 50, -80, 120, 0, 60, 70, 110]"),  
            ([0, 0, 0, 0, 0, 0, 0, 0], 0, "default"), 
            ([0, 0, 0, 0, 0, 0, 0, 0], 0, "default"),  
            ([0, 0, 0, 0, 0, 0, 0, 0], 0, "default"),         
        ];

        println!("Empieza el test");

        x.rst.next = true;
        x.clk.next = Clock { clk: false };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.clk.next = Clock { clk: true };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.rst.next = false;
        x.clk.next = Clock { clk: false };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.clk.next = Clock { clk: true };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;
        x.valid_in.next = true;
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
    let _ = fs::remove_file("test_maximoVectorSegmentacion.v");

    sim.run_to_file(Box::new(uut), 100_000, "maximoVectorSegmentacionWave.vcd")
        .map_err(|e| anyhow!("{:?}", e))?;

    Ok(())
}
