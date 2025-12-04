use rust_hdl::prelude::*;
use rust_hdl_core::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

// Definición del flip-flop EdgeDFF
#[derive(Clone, Debug, LogicBlock, Default)]
pub struct EdgeDFF<T: Synth> {
    pub d: Signal<In, T>,
    pub q: Signal<Out, T>,
    pub clk: Signal<In, Clock>,
}

impl<T: Synth> EdgeDFF<T> {
    pub fn new(init: T) -> EdgeDFF<T> {
        Self {
            d: Signal::default(),
            q: Signal::new_with_default(init),
            clk: Signal::default(),
        }
    }
}

impl<T: Synth> Logic for EdgeDFF<T> {
    fn update(&mut self) {
        if self.clk.pos_edge() {
            self.q.next = self.d.val()
        }
    }
    fn connect(&mut self) {
        self.q.connect();
    }
    fn hdl(&self) -> Verilog {
        Verilog::Custom(format!(
            "\
initial begin
   q = {:x};
end

always @(posedge clk) q <= d;",
            self.q.verilog()
        ))
    }
    fn timing(&self) -> Vec<rust_hdl_core::timing::TimingInfo> {
        vec![rust_hdl_core::timing::TimingInfo {
            name: "edge_ff".to_string(),
            clock: "clk".to_string(),
            inputs: vec!["d".into()],
            outputs: vec!["q".into()],
        }]
    }
}

// Máquina de estados para la multiplicación de matrices 2x2
#[derive(Copy, Clone, PartialEq, Eq, Debug, LogicState)]
enum State {
    Idle,
    Calc1,
    Calc2,
    Calc3,
    Calc4,
    Done,
}

#[derive(LogicBlock, Clone)]
struct MultiplicacionMatricesSecuencial {
    pub clk: Signal<In, Clock>,
    pub rst: Signal<In, Bit>,
    pub start: Signal<In, Bit>,
    pub a: [Signal<In, Signed<4>>; 4],
    pub b: [Signal<In, Signed<4>>; 4],
    pub result: [Signal<Out, Signed<32>>; 4],
    pub done: Signal<Out, Bit>,
    state: EdgeDFF<State>,
    temp1: EdgeDFF<Signed<32>>,
    temp2: EdgeDFF<Signed<32>>,
}

impl Default for MultiplicacionMatricesSecuencial {
    fn default() -> Self {
        let instance = Self {
            clk: Default::default(),
            rst: Default::default(),
            start: Default::default(),
            a: Default::default(),
            //a[1]: Default::default(),
            //a[2]: Default::default(),
            //a[3]: Default::default(),
            b: Default::default(),
            //b[1]: Default::default(),
            //b[2]: Default::default(),
            //b[3]: Default::default(),
            done: Default::default(),
            result: Default::default(),
            //result[1]: Default::default(),
            //result[2]: Default::default(),
            //result[3]: Default::default(),
            state: EdgeDFF::new(State::Idle),
            temp1: EdgeDFF::new(0.into()),
            temp2: EdgeDFF::new(0.into()),
        };
        instance
    }
}

impl Logic for MultiplicacionMatricesSecuencial {
    #[hdl_gen]
    fn update(&mut self) {
        // Enlazamos el reloj
        self.state.clk.next = self.clk.val();
        self.temp1.clk.next = self.clk.val();
        self.temp2.clk.next = self.clk.val();

        self.done.next = false.into();
        
        // Si el reset está activo, volvemos al estado Idle
        if self.rst.val() {
            // Inicializamos el estado
            self.result[0].next = 0.into();
            self.result[1].next = 0.into();
            self.result[2].next = 0.into();
            self.result[3].next = 0.into();
            self.state.d.next = State::Idle;
            self.temp1.d.next = 0.into();
            self.temp2.d.next = 0.into();
        } else {
            match self.state.q.val() {
                State::Idle => {
                    if self.start.val() {
                        self.state.d.next = State::Calc1;
                    }
                }
                State::Calc1 => {
                    self.temp1.d.next = signed_bit_cast::<16, 4>(self.a[0].val()) * signed_bit_cast::<16, 4>(self.b[0].val());
                    self.temp2.d.next = signed_bit_cast::<16, 4>(self.a[1].val()) * signed_bit_cast::<16, 4>(self.b[2].val());
                    self.state.d.next = State::Calc2;
                }
                State::Calc2 => {
                    self.result[0].next = self.temp1.q.val() + self.temp2.q.val();
                    self.temp1.d.next = signed_bit_cast::<16, 4>(self.a[0].val()) * signed_bit_cast::<16, 4>(self.b[1].val());
                    self.temp2.d.next = signed_bit_cast::<16, 4>(self.a[1].val()) * signed_bit_cast::<16, 4>(self.b[3].val());
                    self.state.d.next = State::Calc3;
                }
                State::Calc3 => {
                    self.result[1].next = self.temp1.q.val() + self.temp2.q.val();
                    self.temp1.d.next = signed_bit_cast::<16, 4>(self.a[2].val()) * signed_bit_cast::<16, 4>(self.b[0].val());
                    self.temp2.d.next = signed_bit_cast::<16, 4>(self.a[3].val()) * signed_bit_cast::<16, 4>(self.b[2].val());
                    self.state.d.next = State::Calc4;
                }
                State::Calc4 => {
                    self.result[2].next = self.temp1.q.val() + self.temp2.q.val();
                    self.temp1.d.next = signed_bit_cast::<16, 4>(self.a[2].val()) * signed_bit_cast::<16, 4>(self.b[1].val());
                    self.temp2.d.next = signed_bit_cast::<16, 4>(self.a[3].val()) * signed_bit_cast::<16, 4>(self.b[3].val());
                    self.state.d.next = State::Done;
                }
                State::Done => {
                    self.result[3].next = self.temp1.q.val() + self.temp2.q.val();
                    self.done.next = true.into();
                    self.state.d.next = State::Idle;
                }
            }
        }
    }
}

fn main() {
    // Generar el código Verilog
    let mut uut = MultiplicacionMatricesSecuencial::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module MultiplicacionMatricesSecuencial(");
    let file_path = "multiplicacionMatricesSecuencialRust.v";

    // Escribir el código Verilog a un archivo
    let mut file = std::fs::File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}

#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    std::fs::write("test_multiplicacionMatricesSecuencial.v", tb).unwrap();

    let output = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_multiplicacionMatricesSecuencial.v"])
        .output()?;

    println!("\niverilog output: {:?}", output);

    if !output.status.success() {
        return Err(anyhow!(
            "iverilog falló: {}", String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output = std::process::Command::new("vvp")
        .arg("test_tb.vvp")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into())
}

// Test secuencial de multiplicación de matrices
#[test]
fn test_multiplicacion_matrices_sec() -> anyhow::Result<()> {
    let mut uut = MultiplicacionMatricesSecuencial::default();
    uut.connect_all();

    //Prueba del código en verilog para pasarlo al iverilog
    let verilog_tb = r#"
module test;

    // Entradas
    reg signed [3:0] a0, a1, a2, a3;
    reg signed [3:0] b0, b1, b2, b3;
    reg clk;
    reg rst;
    reg start;
    
    // Salidas
    wire done;
    wire signed [31:0] result0, result1, result2, result3;

    // Instancia del módulo
    MultiplicacionMatricesSecuencial uut (
        .clk(clk),
        .rst(rst),
        .start(start),
        .a$0(a0), .a$1(a1), .a$2(a2), .a$3(a3),
        .b$0(b0), .b$1(b1), .b$2(b2), .b$3(b3),
        .done(done),
        .result$0(result0), .result$1(result1),
        .result$2(result2), .result$3(result3)
    );


    // Generador de reloj
    initial begin
        clk = 0;
        forever #5 clk = ~clk; // Periodo de 10ns
    end

    initial begin
        // Inicialización
        rst = 1; start = 0;
        a0 = 0; a1 = 0; a2 = 0; a3 = 0;
        b0 = 0; b1 = 0; b2 = 0; b3 = 0;
        #20;
        rst = 0;

        // Esperamos un poco
        #10;

        // Caso 1: Máximo producto posible
        $display("=== Test Máximo ===");
        a0 = -8; a1 = -8; a2 = -8; a3 = -8;
        b0 = -8; b1 = -8; b2 = -8; b3 = -8;

        start = 1;
        #10 start = 0;
        
        wait(done);
        #10
        $display("Matriz A:");
        $display("| %d %d |", a0, a1);
        $display("| %d %d |", a2, a3);
        $display("Matriz B:");
        $display("| %d %d |", b0, b1);
        $display("| %d %d |", b2, b3);
        $display("Resultado C:");
        $display("| %d %d |", result0, result1);
        $display("| %d %d |", result2, result3);
        #10;
        $display("// Esperado:");
        $display("// | 128 128 |");
        $display("// | 128 128 |");

        // Caso 2: Mínimo producto posible
        $display("\n=== Test Mínimo ===");
        a0 = -8; a1 = -8; a2 = -8; a3 = -8;
        b0 = 7; b1 = 7; b2 = 7; b3 = 7;
        start = 1;
        #10 start = 0;

        wait(done);
        #10;
        $display("Matriz A:");
        $display("| %d %d |", a0, a1);
        $display("| %d %d |", a2, a3);
        $display("Matriz B:");
        $display("| %d %d |", b0, b1);
        $display("| %d %d |", b2, b3);
        $display("Resultado C:");
        $display("| %d %d |", result0, result1);
        $display("| %d %d |", result2, result3);
        $display("// Esperado:");
        $display("// | -112 -112 |");
        $display("// | -112 -112 |");

        // Caso 3: Mix de positivos y negativos
        $display("\n=== Test Mix ===");
        a0 = 4; a1 = -3; a2 = -2; a3 = 5;
        b0 = -4; b1 = 2; b2 = 3; b3 = -5;
        start = 1;
        #10 start = 0;

        wait(done);
        #10;
        $display("Matriz A:");
        $display("| %d %d |", a0, a1);
        $display("| %d %d |", a2, a3);
        $display("Matriz B:");
        $display("| %d %d |", b0, b1);
        $display("| %d %d |", b2, b3);
        $display("Resultado C:");
        $display("| %d %d |", result0, result1);
        $display("| %d %d |", result2, result3);
        $display("// Esperado:");
        $display("// | -25 23 |");
        $display("// | 23 -29 |");

        // Caso 4: Multiplicación con ceros
        $display("\n=== Test Ceros ===");
        a0 = 0; a1 = 5; a2 = -3; a3 = 0;
        b0 = 0; b1 = 7; b2 = -1; b3 = 0;
        start = 1;
        #10 start = 0;

        wait(done);
        #10;
        $display("Matriz A:");
        $display("| %d %d |", a0, a1);
        $display("| %d %d |", a2, a3);
        $display("Matriz B:");
        $display("| %d %d |", b0, b1);
        $display("| %d %d |", b2, b3);
        $display("Resultado C:");
        $display("| %d %d |", result0, result1);
        $display("| %d %d |", result2, result3);
        $display("// Esperado:");
        $display("// | -5 0 |");
        $display("// | 0 -21 |");

        $finish;
    end

endmodule
"#;

    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    let code = tb.replace("module top(", "module MultiplicacionMatricesSecuencial(");
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("(iverilog) Salida de Verilog:\n{}", sim_output);

    let mut sim = Simulation::<MultiplicacionMatricesSecuencial>::new();
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;

        // Definimos el rango de valores (-2 a 2)
        let valores: Vec<i32> = (-2..=2).collect();

        // Recorremos todas las combinaciones posibles de los valores
        for &a0 in &valores {
            for &a1 in &valores {
                for &a2 in &valores {
                    for &a3 in &valores {
                        for &b0 in &valores {
                            for &b1 in &valores {
                                for &b2 in &valores {
                                    for &b3 in &valores {


                                        //Reset
                                        x.rst.next = false;
                                        x.clk.next = Clock { clk: false };
                                        let x_clone = x.clone();
                                        x = ep.wait(1, x_clone)?;

                                        x.clk.next = Clock { clk: true };
                                        let x_clone = x.clone();
                                        x = ep.wait(1, x_clone)?;

                                        x.clk.next = Clock { clk: false };
                                        let x_clone = x.clone();
                                        x = ep.wait(1, x_clone)?;

                                        x.clk.next = Clock { clk: true };
                                        let x_clone = x.clone();
                                        x = ep.wait(1, x_clone)?;

                                        let mut sol0 = Signed::<32>::from(0 as i64);
                                        let mut sol1 = Signed::<32>::from(0 as i64);
                                        let mut sol2 = Signed::<32>::from(0 as i64);
                                        let mut sol3 = Signed::<32>::from(0 as i64);

                                        // Asignamos los valores de la matriz A
                                        x.a[0].next = Signed::<4>::from(a0 as i64);
                                        x.a[1].next = Signed::<4>::from(a1 as i64);
                                        x.a[2].next = Signed::<4>::from(a2 as i64);
                                        x.a[3].next = Signed::<4>::from(a3 as i64);

                                        // Asignamos los valores de la matriz B
                                        x.b[0].next = Signed::<4>::from(b0 as i64);
                                        x.b[1].next = Signed::<4>::from(b1 as i64);
                                        x.b[2].next = Signed::<4>::from(b2 as i64);
                                        x.b[3].next = Signed::<4>::from(b3 as i64);

                                        x.start.next = true;
                                        // Simulamos un ciclo de reloj
                                        for _cycle in 0..5 {
                                            
                                            x.clk.next = Clock { clk: false };
                                            let x_clone = x.clone();
                                            x = ep.wait(1, x_clone)?;
                                        
                                            x.clk.next = Clock { clk: true };
                                            let x_clone = x.clone();
                                            x = ep.wait(1, x_clone)?;
                                            
                                            sol0 = x.result[0].val();
                                            sol1 = x.result[1].val();
                                            sol2 = x.result[2].val();
                                            sol3 = x.result[3].val();
                                            x.start.next = false;
                                        }

                                        // Calculamos el resultado esperado para la multiplicación de matrices
                                        let c0 = a0 * b0 + a1 * b2;
                                        let c1 = a0 * b1 + a1 * b3;
                                        let c2 = a2 * b0 + a3 * b2;
                                        let c3 = a2 * b1 + a3 * b3;

                                        /*if x.done.val() {
                                        // Imprimimos los resultados para la verificación
                                            println!("\n=== Multiplicación de matrices ===");
                                            println!("A = | {} {} |\n    | {} {} |", a0, a1, a2, a3);
                                            println!("B = | {} {} |\n    | {} {} |", b0, b1, b2, b3);
                                            println!("C = | {} {} |", sol0.bigint(), sol1.bigint());
                                            println!("    | {} {} | // Esperado: | {} {} | | {} {} |",
                                                sol2.bigint(), sol3.bigint(),
                                                c0, c1, c2, c3);
                                        }*/

                                        // Validamos los resultados contra el valor esperado
                                        sim_assert_eq!(ep, sol0, Signed::<32>::from(c0 as i64), x);
                                        sim_assert_eq!(ep, sol1, Signed::<32>::from(c1 as i64), x);
                                        sim_assert_eq!(ep, sol2, Signed::<32>::from(c2 as i64), x);
                                        sim_assert_eq!(ep, sol3, Signed::<32>::from(c3 as i64), x);
                                        
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ep.done(x)?;

        Ok(())
    });

    //Esto es para eliminar el fichero .vvp
    let _e = fs::remove_file("test_tb.vvp");
    let _e = fs::remove_file("test_multiplicacionMatricesSecuencial.v");

    sim.run_to_file(Box::new(uut), 1000000000, "multiplicacionMatricesSecuencialWave.vcd")
        .map_err(|err| anyhow!("{:?}", err))?;

    Ok(())
}
