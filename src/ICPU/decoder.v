module Decoder(
    input [15:0] instruction,
    output reg [3:0] opcode,
    output reg [3:0] dest,
    output reg [3:0] src1,
    output reg [3:0] src2
);
    always @(*) begin
        opcode = instruction[15:12];  // First 4 bits are the opcode
        dest   = instruction[11:8];   // Next 4 bits are destination
        src1   = instruction[7:4];    // Next 4 bits are source1
        src2   = instruction[3:0];    // Last 4 bits are source2
    end
endmodule

always @(opcode) begin
    case (opcode)
        4'b0001: create_node(dest);  // NEW instruction
        4'b0010: link_nodes(dest, src1);  // LINK instruction
        4'b0101: evaluate_node(dest);  // EVAL instruction
        default: no_op();
    endcase
end
