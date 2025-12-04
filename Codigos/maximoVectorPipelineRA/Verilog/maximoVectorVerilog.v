module max_pipeline_tree (
    input clk,
    input rst,
    input valid_in,
    input signed [7:0] data_in_1,  // Entrada de datos 1 (primer valor)
    input signed [7:0] data_in_2,  // Entrada de datos 2 (segundo valor)
    input signed [7:0] data_in_3,  // Entrada de datos 3
    input signed [7:0] data_in_4,  // Entrada de datos 4
    input signed [7:0] data_in_5,  // Entrada de datos 5
    input signed [7:0] data_in_6,  // Entrada de datos 6
    input signed [7:0] data_in_7,  // Entrada de datos 7
    input signed [7:0] data_in_8,  // Entrada de datos 8
    output reg signed [7:0] max_out,  // Resultado final del máximo
    output reg valid_out  // Señal de salida válida
);

    // Registros para almacenar los datos en las diferentes etapas del pipeline
    reg signed [7:0] stage_1_1, stage_1_2, stage_1_3, stage_1_4;
    reg signed [7:0] stage_2_1, stage_2_2;

    reg valid_stage_1, valid_stage_2;

    // Etapa 1: Comparación de pares (entrada directa)
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            stage_1_1 <= 0;
            stage_1_2 <= 0;
            stage_1_3 <= 0;
            stage_1_4 <= 0;
            valid_stage_1 <= 0;
        end else if (valid_in) begin
		    stage_1_1 <= (data_in_1 > data_in_2) ? data_in_1 : data_in_2;
            stage_1_2 <= (data_in_3 > data_in_4) ? data_in_3 : data_in_4;
            stage_1_3 <= (data_in_5 > data_in_6) ? data_in_5 : data_in_6;
            stage_1_4 <= (data_in_7 > data_in_8) ? data_in_7 : data_in_8;
            valid_stage_1 <= 1;
        end
    end

    // Etapa 2: Comparación en pares de la etapa 1
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            stage_2_1 <= 0;
            stage_2_2 <= 0;
            valid_stage_2 <= 0;
        end else if (valid_stage_1) begin
            stage_2_1 <= (stage_1_1 > stage_1_2) ? stage_1_1 : stage_1_2;
            stage_2_2 <= (stage_1_3 > stage_1_4) ? stage_1_3 : stage_1_4;
            valid_stage_2 <= 1;
        end
    end

    // Etapa 3: Comparación de los dos valores de la etapa 2 y Asignacion Final
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            max_out <= 0;
            valid_out <= 0;
        end else if (valid_stage_2) begin
            max_out <= (stage_2_1 > stage_2_2) ? stage_2_1 : stage_2_2;
            valid_out <= 1;
        end else begin
            valid_out <= 0;
        end
    end

endmodule
