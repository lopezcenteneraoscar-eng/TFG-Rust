use rust_hdl::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

// === Módulo de multiplicación de matrices 2x2 ===
#[derive(LogicBlock, Default, Clone)]
pub struct multiplicacionMatrices {
    pub rst: Signal<In, Bit>,
    // Entradas: 2 matrices de 2x2 (4 elementos cada una)
    pub a: [Signal<In, Signed<4>>; 4],
    pub b: [Signal<In, Signed<4>>; 4],
    // Salida: matriz 2x2 resultado
    pub result: [Signal<Out, Signed<32>>; 4],
}

impl Logic for multiplicacionMatrices {
    #[hdl_gen]
    fn update(&mut self) {
        if self.rst.val() {
           self.result[0].next = 0.into(); self.result[1].next = 0.into(); self.result[2].next = 0.into(); self.result[3].next = 0.into(); 
        }
        else {
            self.result[0].next = (signed_bit_cast::<16, 4>(self.a[0].val()) * signed_bit_cast::<16, 4>(self.b[0].val())) + (signed_bit_cast::<16, 4>(self.a[1].val()) * signed_bit_cast::<16, 4>(self.b[2].val()));
            self.result[1].next = (signed_bit_cast::<16, 4>(self.a[0].val()) * signed_bit_cast::<16, 4>(self.b[1].val())) + (signed_bit_cast::<16, 4>(self.a[1].val()) * signed_bit_cast::<16, 4>(self.b[3].val()));
            self.result[2].next = (signed_bit_cast::<16, 4>(self.a[2].val()) * signed_bit_cast::<16, 4>(self.b[0].val())) + (signed_bit_cast::<16, 4>(self.a[3].val()) * signed_bit_cast::<16, 4>(self.b[2].val()));
            self.result[3].next = (signed_bit_cast::<16, 4>(self.a[2].val()) * signed_bit_cast::<16, 4>(self.b[1].val())) + (signed_bit_cast::<16, 4>(self.a[3].val()) * signed_bit_cast::<16, 4>(self.b[3].val()));
        }
    }
}

fn main() {
    // Generar código Verilog y guardarlo en sumadorRust.v
    let mut uut = multiplicacionMatrices::default();
    uut.connect_all();
    let verilog_code = generate_verilog(&uut);
    let code = verilog_code.replace("module top(", "module multiplicacionMatrices(");
    let file_path = "multiplicacionMatricesRust.v";

    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(code.as_bytes()).expect("Error al escribir en el archivo");

    println!("Código Verilog generado y guardado en {}", file_path);
}


// Función para obtener la salida de Icarus Verilog
#[cfg(test)]
fn get_icarus_verilog_output(tb: &str) -> anyhow::Result<String> {
    std::fs::write("test_multiplicacionMatrices.v", tb).unwrap();

    let output = std::process::Command::new("iverilog")
        .args(["-tvvp", "-o", "test_tb.vvp", "test_multiplicacionMatrices.v"])
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

    let mut uut = multiplicacionMatrices::default();
    uut.connect_all();

    //Prueba del codigo en verilog para pasarlo al iverilog
    let verilog_tb = r#"
module test;

    // Entradas
    reg signed [3:0] a0, a1, a2, a3;
    reg signed [3:0] b0, b1, b2, b3;
   
    // Salidas
    wire signed [31:0] result0, result1, result2, result3;

    // Instancia del módulo
    multiplicacionMatrices uut (
       
        .a$0(a0), .a$1(a1), .a$2(a2), .a$3(a3),
        .b$0(b0), .b$1(b1), .b$2(b2), .b$3(b3),
        .result$0(result0), .result$1(result1),
        .result$2(result2), .result$3(result3)
    );

    initial begin
    
        // Multiplicación 1: [1 2; 3 4] x [5 6; 7 8]
        a0 = 1; a1 = 2;
        a2 = 3; a3 = 4;
        b0 = 5; b1 = 6;
        b2 = 7; b3 = -8;
        #10;

        // Esperado:
        // C = [1*5 + 2*7   1*6 + 2*-8 ] = [19 -10]
        //     [3*5 + 4*7   3*6 + 4*-8 ] = [43 -14]
        $display("=== Multiplicación 1 ===");
        $display("A = | %0d %0d |\n    | %0d %0d |", a0, a1, a2, a3);
        $display("B = | %0d %0d |\n    | %0d %0d |", b0, b1, b2, b3);
        $display("C = | %0d %0d |", result0, result1);
        $display("    | %0d %0d | // Esperado: | 19 -10 | | 43 -14 |", result2, result3);

        // Multiplicación 2: [-1 0; 0 -1] x [2 3; 4 5]
        a0 = -1; a1 = 0;
        a2 = 0;  a3 = -1;
        b0 = 2; b1 = 3;
        b2 = 4; b3 = 5;
        #10;
        // Esperado: [(-1*2 + 0*4) (-1*3 + 0*5)] = [-2 -3]
        //           [(0*2 + -1*4) (0*3 + -1*5)] = [-4 -5]
        $display("\n=== Multiplicación 2 ===");
        $display("A = | %0d %0d |\n    | %0d %0d |", a0, a1, a2, a3);
        $display("B = | %0d %0d |\n    | %0d %0d |", b0, b1, b2, b3);
        $display("C = | %0d %0d |", result0, result1);
        $display("    | %0d %0d | // Esperado: | -2 -3 | | -4 -5 |", result2, result3);

        // Multiplicación 3: [3 1; 0 2] x [1 0; 2 1]
        a0 = 3; a1 = 1;
        a2 = 0; a3 = 2;
        b0 = 1; b1 = 0;
        b2 = 2; b3 = 1;
        #10;
        // Esperado: [(3*1 + 1*2) (3*0 + 1*1)] = [5 1]
        //           [(0*1 + 2*2) (0*0 + 2*1)] = [4 2]
        $display("\n=== Multiplicación 3 ===");
        $display("A = | %0d %0d |\n    | %0d %0d |", a0, a1, a2, a3);
        $display("B = | %0d %0d |\n    | %0d %0d |", b0, b1, b2, b3);
        $display("C = | %0d %0d |", result0, result1);
        $display("    | %0d %0d | // Esperado: | 5 1 | | 4 2 |", result2, result3);

        // Multiplicación 4: [2 2; 1 3] x [1 4; 2 0]
        a0 = 2; a1 = 2;
        a2 = 1; a3 = 3;
        b0 = 1; b1 = 4;
        b2 = 2; b3 = 0;
        #10;
        // Esperado: [(2*1 + 2*2) (2*4 + 2*0)] = [6 8]
        //           [(1*1 + 3*2) (1*4 + 3*0)] = [7 4]
        $display("\n=== Multiplicación 4 ===");
        $display("A = | %0d %0d |\n    | %0d %0d |", a0, a1, a2, a3);
        $display("B = | %0d %0d |\n    | %0d %0d |", b0, b1, b2, b3);
        $display("C = | %0d %0d |", result0, result1);
        $display("    | %0d %0d | // Esperado: | 6 8 | | 7 4 |", result2, result3);
	
	    // Multiplicación 5: [-8 -8; -8 -8] x [-8 -8; -8 -8]
        a0 = -8; a1 = -8;
        a2 = -8; a3 = -8;
        b0 = -8; b1 = -8;
        b2 = -8; b3 = -8;
        #10;
        // Esperado:
        // C = [-8*-8 + -8*-8   -8*-8 + -8*-8 ] = [128 128]
        //     [-8*-8 + -8*-8   -8*-8 + -8*-8 ] = [128 128]
        $display("\n=== Multiplicación 5: Máximo ===");
        $display("A = | %0d %0d |\n    | %0d %0d |", a0, a1, a2, a3);
        $display("B = | %0d %0d |\n    | %0d %0d |", b0, b1, b2, b3);
        $display("C = | %0d %0d |", result0, result1);
        $display("    | %0d %0d | // Esperado: | 128 128 | | 128 128 |", result2, result3);

        // Multiplicación 6: [-8 -8; -8 -8] x [7 7; 7 7]
        a0 = -8; a1 = -8;
        a2 = -8; a3 = -8;
        b0 = 7;  b1 = 7;
        b2 = 7;  b3 = 7;
        #10;
        // Esperado:
        // C = [-8*7 + -8*7   -8*7 + -8*7 ] = [-112 -112]
        //     [-8*7 + -8*7   -8*7 + -8*7 ] = [-112 -112]
        $display("\n=== Multiplicación 6: Mínimo ===");
        $display("A = | %0d %0d |\n    | %0d %0d |", a0, a1, a2, a3);
        $display("B = | %0d %0d |\n    | %0d %0d |", b0, b1, b2, b3);
        $display("C = | %0d %0d |", result0, result1);
        $display("    | %0d %0d | // Esperado: | -112 -112 | | -112 -112 |", result2, result3);


        $finish;
    end

endmodule
"#;
    //Esto es para concatenar el testbench y el codigo en verilog del sumador
    let tb = format!("{verilog_tb} {}", generate_verilog(&uut));
    //Remplazamos el nombre module top que tiene por defecto y se lo pasamos al iverilog
    let code = tb.replace("module top(", "module multiplicacionMatrices(");
    //Esto le pasa al iverilog el testbench y luego nos muestra la salida
    let sim_output = get_icarus_verilog_output(&code)?;
    println!("(iverilog) Salida de Verilog:\n{}", sim_output);  
    
    
    
    // Generamos la simulación y le añadimos el testbench
    let mut sim = Simulation::<multiplicacionMatrices>::new();
    sim.add_testbench(move |mut ep| {
        let mut x = ep.init()?;
        /*
        // Definimos los casos de prueba (A, B, C esperado)
        let test_cases = vec![
            // Multiplicación 5 (Máximo)
            (
                [-7, -7, -7, -7], [-7, -7, -7, -7], [98, 98, 98, 98]
            ),
            // Multiplicación 6 (Mínimo)
            (
                [-7, -7, -7, -7], [7, 7, 7, 7], [-98, -98, -98, -98]
            ),
        ];

        for (idx, (a_vals, b_vals, expected_c)) in test_cases.iter().enumerate() {
            println!("\n=== Multiplicación {} ===", idx + 1);

            // Cargamos la matriz A
            for i in 0..4 {
                x.a[i].next = Signed::<4>::from(a_vals[i]);
            }
            // Cargamos la matriz B
            for i in 0..4 {
                x.b[i].next = Signed::<4>::from(b_vals[i]);
            }

            let x_clone = x.clone();
            let x = ep.wait(1, x_clone)?;

            println!("A = | {} {} |\n    | {} {} |", a_vals[0], a_vals[1], a_vals[2], a_vals[3]);
            println!("B = | {} {} |\n    | {} {} |", b_vals[0], b_vals[1], b_vals[2], b_vals[3]);
            println!("C = | {} {} |", x.result[0].val().bigint(), x.result[1].val().bigint());
            println!("    | {} {} | // Esperado: | {} {} | | {} {} |",
                x.result[2].val().bigint(), x.result[3].val().bigint(),
                expected_c[0], expected_c[1], expected_c[2], expected_c[3]);

            // Aquí puedes hacer asserts si quieres (opcional)
            sim_assert_eq!(ep, x.result[0].val(), Signed::<32>::from(expected_c[0]), x);
            sim_assert_eq!(ep, x.result[1].val(), Signed::<32>::from(expected_c[1]), x);
            sim_assert_eq!(ep, x.result[2].val(), Signed::<32>::from(expected_c[2]), x);
            sim_assert_eq!(ep, x.result[3].val(), Signed::<32>::from(expected_c[3]), x);
        }*/

        println!("Iniciando simulación de multiplicación de matrices...");

        let valores: Vec<i32> = (-2..=2).collect(); // Limita el rango por ahora
        
        x.rst.next = true; 
        ep.wait(1, x.clone())?;
        x.rst.next = false;
        ep.wait(1, x.clone())?;
        
        for &a0 in &valores {
            for &a1 in &valores {
                for &a2 in &valores {
                    for &a3 in &valores {
                        for &b0 in &valores {
                            for &b1 in &valores {
                                for &b2 in &valores {
                                    for &b3 in &valores {

                                        x.a[0].next = Signed::<4>::from(a0 as i64);
                                        x.a[1].next = Signed::<4>::from(a1 as i64);
                                        x.a[2].next = Signed::<4>::from(a2 as i64);
                                        x.a[3].next = Signed::<4>::from(a3 as i64);

                                        x.b[0].next = Signed::<4>::from(b0 as i64);
                                        x.b[1].next = Signed::<4>::from(b1 as i64);
                                        x.b[2].next = Signed::<4>::from(b2 as i64);
                                        x.b[3].next = Signed::<4>::from(b3 as i64);

                                        let x_clone = x.clone();
                                        let x = ep.wait(1, x_clone)?;

                                        // Calculamos el resultado esperado en software para validar
                                        let c0 = a0 * b0 + a1 * b2;
                                        let c1 = a0 * b1 + a1 * b3;
                                        let c2 = a2 * b0 + a3 * b2;
                                        let c3 = a2 * b1 + a3 * b3;
                                        /*
                                        println!("\n=== Multiplicación de matrices ===");
                                        println!("A = | {} {} |\n    | {} {} |", a0, a1, a2, a3);
                                        println!("B = | {} {} |\n    | {} {} |", b0, b1, b2, b3);
                                        println!("C = | {} {} |", x.result[0].val().bigint(), x.result[1].val().bigint());
                                        println!("    | {} {} | // Esperado: | {} {} | | {} {} |",
                                        x.result[2].val().bigint(), x.result[3].val().bigint(), c0, c1, c2, c3);
                                        */
                                        // Validaciones
                                        sim_assert_eq!(ep, x.result[0].val(), Signed::<32>::from(c0 as i64), x);
                                        sim_assert_eq!(ep, x.result[1].val(), Signed::<32>::from(c1 as i64), x);
                                        sim_assert_eq!(ep, x.result[2].val(), Signed::<32>::from(c2 as i64), x);
                                        sim_assert_eq!(ep, x.result[3].val(), Signed::<32>::from(c3 as i64), x);

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
    let _e = fs::remove_file("test_multiplicacionMatrices.v");

    sim.run_to_file(Box::new(uut), 1000000000, "multiplicacionMatricesWave.vcd")
        .map_err(|err| anyhow!("{:?}", err))?;
    
    Ok(())
}
