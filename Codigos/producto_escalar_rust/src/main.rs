//Producto escalar en Rust
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

//Multiplicador
#[derive(LogicBlock, Default, Clone)]
pub struct Mul8x8 {
    pub a: Signal<In, Bits<8>>,
    pub b: Signal<In, Bits<8>>,
    pub resul: Signal<Out, Bits<16>>,
}

impl Logic for Mul8x8 {

    #[hdl_gen]
    fn update(&mut self) {
        self.resul.next = 0.into();

        if self.b.val().get_bit(0) {
            self.resul.next = self.resul.val() + bit_cast::<16, 8>(self.a.val());
        }
        if self.b.val().get_bit(1) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 1);
        }
        if self.b.val().get_bit(2) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 2);
        }
        if self.b.val().get_bit(3) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 3);
        }
        if self.b.val().get_bit(4) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 4);
        }
        if self.b.val().get_bit(5) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 5);
        }
        if self.b.val().get_bit(6) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 6);
        }
        if self.b.val().get_bit(7) {
            self.resul.next = self.resul.val() + (bit_cast::<16, 8>(self.a.val()) << 7);
        }
    }
}

//Producto escalar
#[derive(LogicBlock, Default, Clone)]
pub struct productoEscalar {
    pub clock: Signal<In, Clock>,
    pub reset: Signal<In, Bit>,
    pub start: Signal<In, Bit>,        //señal para iniciar el cálculo
    pub a: Signal<In, Bits<8>>,        
    pub b: Signal<In, Bits<8>>,
    pub valid: Signal<In, Bit>,        //indica si los datos de entrada son válidos
    pub result: Signal<Out, Bits<16>>, //resultado del producto escalar
    pub busy: Signal<Out, Bit>,
    
    //registros internos
    accumulator: EdgeDFF<Bits<16>>,        //acumulador del producto escalar
    counter: EdgeDFF<Bits<8>>,             //contador de ciclos/índice
    e_busy: EdgeDFF<Bit>,                    //estado de ocupado
    max_count: Signal<Local, Bits<8>>,
    mul: Mul8x8,
    
}


impl Logic for productoEscalar {
    #[hdl_gen]
    fn update(&mut self) { //solo cosas con self (señales internas)
        //enlazamos el reloj
        //dff_setup!(self, clock, accumulator, counter, busy);
        self.accumulator.clk.next = self.clock.val();
        self.counter.clk.next = self.clock.val();
        self.e_busy.clk.next = self.clock.val();

        //valor por defecto, no creo que haga falta pero por si acaso
        self.accumulator.d.next = self.accumulator.q.val();
        self.counter.d.next = self.counter.q.val();
        self.e_busy.d.next = self.e_busy.q.val();

        self.max_count.next = 4.into(); //debería ser N-1 pero no consigo generalizarlo
        self.result.next = 0.into();

        if self.reset.val() {
            self.accumulator.d.next = 0.into();
            self.counter.d.next = 0.into();
            self.e_busy.d.next = false;
            self.busy.next = false;
        } 
        else if !self.e_busy.q.val() { //si no está ocupado
            if self.start.val() { //esperamos a que se inicie el cálculo
                self.e_busy.d.next = true;
                self.counter.d.next = 0.into();
                self.accumulator.d.next = 0.into();
                self.busy.next = true;
            }
        } 
        else {
            //procesamos los datos
            if self.valid.val() { //si los datos son válidos(en la simulacion poner siempre a 1)
                //conectamos entradas al multiplicador y esperamos el resultado
                self.mul.a.next = self.a.val();
                self.mul.b.next = self.b.val();
                
                self.accumulator.d.next = self.accumulator.q.val() + self.mul.resul.val(); //sumamos datos
                self.counter.d.next = self.counter.q.val() + 1; //sumamos 1 al cont

                if self.counter.q.val() == self.max_count.val() { //si ya hemos multiplicado todo
                    self.result.next = self.accumulator.q.val().into();
                    self.e_busy.d.next = false; //señalizamos que ya no está ocupado
                    self.busy.next = false;
                }
            }
        }
    }
}

fn main() {
    // Generar código Verilog y guardarlo en sumadorRust.v
    let mut uut = productoEscalar::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module productoEscalar(");
    let file_path = "productoEscalarRust.v";

    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}


// Función para obtener la salida de Icarus Verilog
#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    std::fs::write("test_productoEscalar.v", tb).unwrap();

    let output = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_productoEscalar.v"])
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
fn test_multiplicacion_matrices() -> anyhow::Result<()> {

    let mut uut = productoEscalar::default();
    uut.connect_all();

    let verilog_tb = r#"
    module test_productoEscalar;

        reg clock;
        reg reset;
        reg start;
        reg [7:0] a, b;
        reg valid;
        wire [15:0] result;
        wire busy;

        // Instanciamos el módulo bajo prueba
        productoEscalar uut (
            .clock(clock),
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
            #5 clock = ~clock;
        end

        // Inicialización y prueba
        initial begin
            // Inicializamos las señales
            clock = 0;
            reset = 1;
            start = 0;
            valid = 0;
            a = 8'b0;
            b = 8'b0;

            // Esperamos un ciclo de reloj
            #10;
            reset = 0;

            // Iniciamos el cálculo
            start = 1;
            #10;

            start = 0;
            valid = 1;

            a = 8'd3;  b = 8'd4;  #10;
            $display("a = %d b = %d a*b = 12", a, b);
            a = 8'd5;  b = 8'd6;  #10;
            $display("a = %d b = %d a*b = 30", a, b);
            a = 8'd7;  b = 8'd8;  #10;
            $display("a = %d b = %d a*b = 48", a, b);
            a = 8'd9;  b = 8'd10; #10;
            $display("a = %d b = %d a*b = 90", a, b);
            $display("Resultado Esperado :12 + 30 + 56 + 90 = 188");
            
            // Mostramos el resultado
            $display("Resultado final: %d", result);
            $finish;
        end
    endmodule
    "#;

    //Esto es para concatenar el testbench y el codigo en verilog del sumador
    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    //Remplazamos el nombre module top que tiene por defecto y se lo pasamos al iverilog
    let code = tb.replace("module top(", "module productoEscalar(");
    //Esto le pasa al iverilog el testbench y luego nos muestra la salida
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("(iverilog) Salida de Verilog:\n{}", sim_output);    

    // Generamos la simulación y le añadimos el testbench
    let mut sim = Simulation::<productoEscalar>::new();
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;

        // Entradas sincronizadas como en Verilog
        let a_vals = vec![3, 5, 7, 9];
        let b_vals = vec![4, 6, 8, 10];

        // 1. Reset (1 ciclo)
        x.reset.next = true;
        x.clock.next = Clock { clk: false };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.clock.next = Clock { clk: true };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.reset.next = false;

        // 2. Start (1 ciclo)
        x.start.next = true;
        x.clock.next = Clock { clk: false };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;
        x.clock.next = Clock { clk: true };
        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        x.start.next = false;

        // 3. Enviar valores válidos (4 ciclos)
        x.valid.next = true;
        for (&a, &b) in a_vals.iter().zip(b_vals.iter()) {
            x.a.next = Bits::<8>::from(a);
            x.b.next = Bits::<8>::from(b);
            println!("a = {} b = {} a*b = {}", a, b, a*b);
            x.clock.next = Clock { clk: false };
            let x_clone = x.clone();
            x = ep.wait(1, x_clone)?;
            x.clock.next = Clock { clk: true };
            let x_clone = x.clone();
            x = ep.wait(1, x_clone)?;
        }

        println!("Resultado Esperado: 12 + 30 + 56 + 90 = 188");
        // 5. Mostrar resultado final
        let result = u64::from(x.result.val());
        println!("Resultado final: {}", result);

        // Resultado esperado: 3×4 + 5×6 + 7×8 + 9×10 = 12 + 30 + 56 + 90 = 188
        sim_assert_eq!(ep, x.result.val(), Bits::<16>::from(188), x);

        ep.done(x)
    });


    /*
    
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;
	
        // Definimos algunos valores de prueba para los vectores
        let a_vals = vec![1, 2, 3];  // Ejemplos de valores para el vector A
        let b_vals = vec![1, 2, 3];  // Ejemplos de valores para el vector B

        let x_clone = x.clone();
        x = ep.wait(1, x_clone)?;

        // Iteramos sobre las combinaciones de valores de a y b
        for &a in &a_vals {
            for &b in &b_vals {
                // Cargamos los valores de entrada para el producto escalar
                x.a.next = Bits::<8>::from(a as u64);
                x.b.next = Bits::<8>::from(b as u64);

                x.valid.next = true;
                x.clock.next = Clock { clk: false };
                let x_clone = x.clone();
                x = ep.wait(1, x_clone)?;

                x.clock.next = Clock { clk: false };
                let x_clone = x.clone();
                x = ep.wait(1, x_clone)?;


                // Calculamos el resultado esperado en software (producto escalar)
                let expected_result = a * b;
                println!("Num {} Salida {:?}", expected_result, u64::from(x.result.val()));
                // Validamos el resultado en hardware con el esperado
                //sim_assert_eq!(ep, x.result.val(), Bits::<16>::from(expected_result as u64), x);
            }
        }
	        
        ep.done(x)?;

        Ok(())
    });*/

    //Esto es para eliminar el fichero .vvp
    let _e = fs::remove_file("test_tb.vvp");
    let _e = fs::remove_file("test_productoEscalar.v");

    sim.run_to_file(Box::new(uut), 1000000000, "productoEscalarWave.vcd")
        .map_err(|err| anyhow!("{:?}", err))?;
    
    Ok(())
}
