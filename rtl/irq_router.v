module irq_router (
	input  wire uart_irq,
	input  wire audio_irq,
	input  wire audio_dma_irq,
	input  wire sdc_data_irq,
	input  wire sdc_cmd_irq,
	input  wire gpio_switches_irq,
	input  wire gpio_buttons_irq,

	output wire [31:0] sources
);

	assign sources[0] = 1'b0;
	
	assign sources[1] = uart_irq;
	assign sources[2] = audio_irq;
	assign sources[3] = audio_dma_irq;
	assign sources[4] = sdc_data_irq;
	assign sources[5] = sdc_cmd_irq;
	assign sources[6] = gpio_switches_irq;
	assign sources[7] = gpio_buttons_irq;
	
	assign sources[8]  = 1'b0;
	assign sources[9]  = 1'b0;
	assign sources[10] = 1'b0;
	assign sources[11] = 1'b0;
	assign sources[12] = 1'b0;
	assign sources[13] = 1'b0;
	assign sources[14] = 1'b0;
	assign sources[15] = 1'b0;
	assign sources[16] = 1'b0;
	assign sources[17] = 1'b0;
	assign sources[18] = 1'b0;
	assign sources[19] = 1'b0;
	assign sources[20] = 1'b0;
	assign sources[21] = 1'b0;
	assign sources[22] = 1'b0;
	assign sources[23] = 1'b0;
	assign sources[24] = 1'b0;
	assign sources[25] = 1'b0;
	assign sources[26] = 1'b0;
	assign sources[27] = 1'b0;
	assign sources[28] = 1'b0;
	assign sources[29] = 1'b0;
	assign sources[30] = 1'b0;
	assign sources[31] = 1'b0;

endmodule
