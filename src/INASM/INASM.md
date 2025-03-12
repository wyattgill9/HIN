Explanation of Instructions:
Instruction	Meaning:

NEW dst val - Creates a new node dst with value val
LINK src dst - Connects two nodes
DEL node - Deletes a node
MATCH pattern action -	Matches a rule pattern and applies action
EVAL node - Triggers evaluation on a node
CALL @label arg1 arg2 - Calls a function with arguments
RETURN val - Returns a value from a function


Example Opcodes
Instruction	Opcode Encoding
NEW dst val	0001	0001 0001 0011 0000 (Create node 3 with value 0)
LINK src dst	0010	0010 0011 0100 0000 (Link node 3 to node 4)
DEL node	0011	0011 0011 0000 0000 (Delete node 3)
MATCH pattern action	0100	TBD
EVAL node	0101	0101 0011 0000 0000 (Evaluate node 3)
CALL label arg1 arg2	0110	TBD