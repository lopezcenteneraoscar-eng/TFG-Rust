module tb_sumadorVerilog;
	reg signed [7:0] a, b;
	wire signed [8:0] sum;

	sumador uut (
		.a(a), 
		.b(b), 
		.sum(sum)
	);

	initial begin
		$dumpfile("sumadorVerilogWave.vcd");
		$dumpvars(0, tb_sumadorVerilog);

		$monitor("a=%d, b=%d, sum=%d", a, b, sum);

		a = 10; b = 20; #10
		
		a = -50; b = 75; #10
		
		a = -50; b = -50; #10
		
		a = 127; b = -127; #10
		
		#10
		
		$finish;
	end
endmodule


