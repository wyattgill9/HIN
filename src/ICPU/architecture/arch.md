


the cpu will follow this fetch-decode-execute cycle:
    Fetch – Read the next instruction from memory.
    Decode – Break it into opcode and operands.
    Execute – Perform the operation (create a node, link nodes, etc.).
    Store results – Update memory / registers.

plan asembler:

Takes the 16-bit binary instruction as input.
Extracts the opcode and operands.
Triggers the appropriate control signals.

