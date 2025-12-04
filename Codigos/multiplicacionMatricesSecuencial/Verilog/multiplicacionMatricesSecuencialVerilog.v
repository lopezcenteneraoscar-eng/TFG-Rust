module multiplicacionMatricesSecuencial (
	input clk,
	input rst,
	input start,
	input signed [3:0] a11, a12, a21, a22,
	input signed [3:0] b11, b12, b21, b22,
	output reg done,
	output reg signed[8:0] c11, c12, c21, c22
);

reg [2:0] state;

reg signed [8:0] temp1, temp2;

localparam IDLE = 3'd0,
	   CALC1 = 3'd1,
	   CALC2 = 3'd2,
	   CALC3 = 3'd3,
	   CALC4 = 3'd4,
	   DONE = 3'd5;

always @(posedge clk or posedge rst) begin
	if (rst) begin
		state <= IDLE;
		done <= 0;
		c11 <= 0; c12 <= 0; c21 <= 0; c22 <= 0;
	end else begin
		case (state)
			IDLE: begin
				done <= 0;
				if (start)
					state <= CALC1;
			end

			CALC1: begin
				temp1 <= a11 * b11;
				temp2 <= a12 * b21;
				state <= CALC2;
			end

			CALC2: begin
				c11 <= temp1 + temp2;
				temp1 <= a11 * b12;
				temp2 <= a12 * b22;
				state <= CALC3;
			end
	
			CALC3: begin
				c12 <= temp1 + temp2;
				temp1 <= a21 * b11;
				temp2 <= a22 * b21;
				state <= CALC4;
			end

			CALC4: begin
				c21 <= temp1 + temp2;
				temp1 <= a21 * b12;
				temp2 <= a22 * b22;
				state <= DONE;
			end

			DONE: begin
				c22 <= temp1 + temp2;
				done <= 1;
				state <= IDLE;
			end
		endcase
	end
end
endmodule
