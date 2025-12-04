use rust_hdl::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

// Tu Flip-Flop personalizado
use rust_hdl_core::prelude::*;
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

//-------------------- Máquina de estados modificada --------------------

#[derive(Copy, Clone, PartialEq, Eq, Debug, LogicState)]
enum State {
    S0,
    S1,
    S2,
    S3,
    S4,
}

#[derive(LogicBlock, Clone)]
struct ReconocedorPatrones {
    pub clk: Signal<In, Clock>,
    pub rst: Signal<In, Bit>,
    pub entrada: Signal<In, Bit>,
    pub salida: Signal<Out, Bit>,
    state: EdgeDFF<State>, // <--- Usamos tu EdgeDFF personalizado
}

impl Default for ReconocedorPatrones {
    fn default() -> Self {
        let instance = Self {
            clk: Default::default(),
            rst: Default::default(),
            entrada: Default::default(),
            salida: Default::default(),
            state: EdgeDFF::new(State::S0),
        };
        instance
    }
}

impl Logic for ReconocedorPatrones {
    #[hdl_gen]
    fn update(&mut self) {
        // Enlazamos reloj
        self.state.clk.next = self.clk.val();

        // Valor por defecto
        self.salida.next = false;
        self.state.d.next = self.state.q.val(); // Mantiene el estado actual

        if self.rst.val() {
            self.state.d.next = State::S0;
        } else {
            match self.state.q.val() {
                State::S0 => {
                    if self.entrada.val() {
                        self.state.d.next = State::S1;
                    }
                }
                State::S1 => {
                    if !self.entrada.val() {
                        self.state.d.next = State::S2;
                    }
                }
                State::S2 => {
                    if self.entrada.val() {
                        self.state.d.next = State::S3;
                    } else {
                        self.state.d.next = State::S0;
                    }
                }
                State::S3 => {
                    if self.entrada.val() {
                        self.state.d.next = State::S4;
                    } else {
                        self.state.d.next = State::S2;
                    }
                }
                State::S4 => {
                    self.salida.next = true;
                    self.state.d.next = State::S0;
                }
            }
        }
    }
}

fn main() {
    // Generar código Verilog y guardarlo en sumadorRust.v
    let mut uut = ReconocedorPatrones::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module reconocedorPatrones(");
    let file_path = "reconocedorPatronesRust.v";

    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}

// Función para obtener la salida de Icarus Verilog
#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    std::fs::write("test_reconocedorPatrones.v", tb).unwrap();

    let output = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_reconocedorPatrones.v"])
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

// Test con RustHDL y comparación con Verilog
#[test]
fn test_reconocedor_patrones() -> anyhow::Result<()> {

    let mut uut = ReconocedorPatrones::default();
    uut.connect_all();

    //Prueba del codigo en verilog para pasarlo al iverilog
    let verilog_tb = r#"
module test;
  reg clk, rst, entrada;
  wire salida;

  reconocedorPatrones uut(.clk(clk), .rst(rst), .entrada(entrada), .salida(salida));

    // Generador de reloj (período de 10 ns)
    always #5 clk = ~clk;

    task apply_input;
        input val;
        begin
            entrada = val;
            #10;
            $display("t=%0t | entrada=%b | salida=%b", $time, entrada, salida);
        end
    endtask

    // Proceso de prueba
    initial begin
        // Inicialización
        clk = 0;
        rst = 1;
        entrada = 0;
        #10; // Esperar 10 ns

        rst = 0; // Liberar el reset
        #10;

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
        #20;
        $finish; // Finaliza la simulación
    end

endmodule
"#;
    //Esto es para concatenar el testbench y el codigo en verilog del sumador
    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    //Remplazamos el nombre module top que tiene por defecto y se lo pasamos al iverilog
    let code = tb.replace("module top(", "module reconocedorPatrones(");
    //Esto le pasa al iverilog el testbench y luego nos muestra la salida
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("(iverilog) Salida de Verilog:\n{}", sim_output);

    // Simulación en RustHDL con VCD
    let mut sim = Simulation::<ReconocedorPatrones>::new();
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;

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

        for bit in [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1] {
            x.entrada.next = bit == 1;
            x.clk.next = Clock { clk: false };
            let x_clone = x.clone();
            x = ep.wait(10, x_clone)?;

            x.clk.next = Clock { clk: true }; //Subida de reloj
            let x_clone = x.clone();
            x = ep.wait(10, x_clone)?;

            println!("Entrada={}, Salida={}", bit, x.salida.val());
        }

        ep.done(x)
    });

    //Esto es para eliminar el fichero .vvp
    let _e = fs::remove_file("test_tb.vvp");
    let _e = fs::remove_file("test_reconocedorPatrones.v");

    sim.run_to_file(Box::new(uut), 180_000, "reconocedorPatronesWave.vcd")
        .map_err(|err| anyhow!("{:?}", err))?;
    
    Ok(())
}
