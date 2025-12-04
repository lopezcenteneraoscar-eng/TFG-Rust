module ArbolSumadoresSegmentacion #(parameter WIDTH = 8) (
	input clk,
	input rstn,
	input [WIDTH-1:0] in0, in1, in2, in3, in4, in5, in6, in7,
	output reg [WIDTH+2:0] result // Resultado Final: suma de los 8 valores
);

//Etapas del pipeline o de la segmentacion
//Etapa 1: suma por pares
reg signed [WIDTH:0] stage1_0, stage1_1, stage1_2, stage1_3; //4 sumas de 9 bits

//Etapa 2: suma de sumas
reg signed [WIDTH+1:0] stage2_0, stage2_1; //2 sumas de 10 bits

//Etapa 3: suma final
reg signed [WIDTH+2:0] stage3; //Resultado final

always @(posedge clk or negedge rstn) begin
	if (!rstn) begin
		stage1_0 <= 0; stage1_1 <= 0; stage1_2 <= 0; stage1_3 <= 0;
		stage2_0 <= 0; stage2_1 <= 0;
		result <= 0;
	end else begin

		//Etapa 1
		stage1_0 <= $signed(in0) + $signed(in1);
		stage1_1 <= $signed(in2) + $signed(in3);
		stage1_2 <= $signed(in4) + $signed(in5);
		stage1_3 <= $signed(in6) + $signed(in7);

		//Etapa 2
		stage2_0 <= $signed(stage1_0) + $signed(stage1_1);
		stage2_1 <= $signed(stage1_2) + $signed(stage1_3);

		//Salida
		result <= stage2_0 + stage2_1;

	end
end

endmodule
