use rust_hdl::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

// Definición del módulo Sumador
#[derive(LogicBlock, Clone)]
struct Sumador {
    pub a: Signal<In, Signed<8>>,   // Entrada A (8 bits)
    pub b: Signal<In, Signed<8>>,   // Entrada B (8 bits)
    pub sum: Signal<Out, Signed<9>>, // Salida Sum (9 bits)
}

impl Default for Sumador {
    fn default() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
            sum: Default::default(),
        }
    }
}

impl Logic for Sumador {
    #[hdl_gen]
    fn update(&mut self) {
        self.sum.next = signed_bit_cast::<9, 8>(self.a.val()) + signed_bit_cast::<9, 8>(self.b.val());
    }
}

fn main() {
    // Generar código Verilog y guardarlo en sumadorRust.v
    let mut uut = Sumador::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module Sumador(");
    let file_path = "sumadorRust.v";

    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}

// Función para obtener la salida de Icarus Verilog
#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    std::fs::write("test_sumador.v", tb).unwrap();

    let output = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_sumador.v"])
        .output()?;

    println!("\niVerilog Falla?: {:?}", output);

    let output = std::process::Command::new("vvp")
        .arg("test_tb.vvp")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into())
}

// Test con RustHDL y comparación con Verilog
#[test]
fn test_sumador() -> anyhow::Result<()> {

    let mut uut = Sumador::default();
    uut.connect_all();

    //Prueba del codigo en verilog para pasarlo al iverilog
    let verilog_tb = r#"
module test;
  reg signed [7:0] a, b;
  wire signed [8:0] sum;

  Sumador uut(.a(a), .b(b), .sum(sum));

  initial begin
    $monitor("a=%d, b=%d, sum=%d", a, b, sum);
    a = -128; b = -128; #10;
    a = -50; b = 75; #10;
    a = -50; b = 50; #10;
    a = 127; b = 127; #10;
    #10
    $finish;
  end
endmodule
"#;
    //Esto es para concatenar el testbench y el codigo en verilog del sumador
    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    //Remplazamos el nombre module top que tiene por defecto y se lo pasamos al iverilog
    let code = tb.replace("module top(", "module Sumador(");
    //Esto le pasa al iverilog el testbench y luego nos muestra la salida
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("(iverilog) Salida de Verilog:\n{}", sim_output);

    //Generamos la simulacion del sumador y le añadimos el testbench
    let mut sim = Simulation::<Sumador>::new();
    sim.add_testbench(move |mut ep| {
        //Iniciamos
        let mut x = ep.init()?;
      
        //Casos que queremos probar  -128 a 127
        
        let test_cases: Vec<(i32, i32)> = (-127..128).flat_map(|a| (-127..128).map(move |b| ((a,b)))).collect();
        
        //Un for probando todos los casos posibles, al se 256 x 256 se desborda la pila, pero se hace asi
        for &(a, b) in test_cases.iter() {
            x.a.next = Signed::<8>::from(a as i64);
            x.b.next = Signed::<8>::from(b as i64);
            let x_clone = x.clone();
            let x = ep.wait(1, x_clone)?;
            //println!("a= {}, b= {}, sum= {}", x.a.val().bigint(), x.b.val().bigint(), x.sum.val().bigint());
            let expected_sum = a + b;
            sim_assert_eq!(ep, x.sum.val(), Signed::<9>::from(expected_sum as i64), x);
        } 
        
        //Terminamos y pintamos OK si no ha fallado
        ep.done(x)?;
        Ok(())
    });

    //Esto es para eliminar el fichero .vvp
    let _e = fs::remove_file("test_tb.vvp");
    let _e = fs::remove_file("test_sumador.v");

    sim.run_to_file(Box::new(uut), 100000, "sumadorWave.vcd")
        .map_err(|err| anyhow!("{:?}", err))?;
    
    Ok(())
}
