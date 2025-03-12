Explanation of Instructions:
Instruction	Meaning:

NEW dst val - Creates a new node dst with value val
LINK src dst - Connects two nodes
DEL node - Deletes a node
MATCH pattern action -	Matches a rule pattern and applies action
EVAL node - Triggers evaluation on a node
CALL @label arg1 arg2 - Calls a function with arguments
RETURN val - Returns a value from a function

| Instruction              | Opcode (4 bits) | Format                | Description                                     |
|--------------------------|-----------------|-----------------------|-------------------------------------------------|
| `NEW dst val`             | `0001` (0x1)    | `0001 dddd vvvv vvvv`  | Create a new node `dst` with value `val`        |
| `LINK src dst`            | `0010` (0x2)    | `0010 ssss dddd 0000`  | Link `src` node to `dst` node                   |
| `DEL node`                | `0011` (0x3)    | `0011 nnnn 0000 0000`  | Delete node `node`                              |
| `MATCH pattern action`    | `0100` (0x4)    | TBD                   | Check if a pattern exists and apply `action`    |
| `EVAL node`               | `0101` (0x5)    | `0101 nnnn 0000 0000`  | Evaluate the interaction of `node`              |
| `RETURN node`             | `0111` (0x7)    | `0111 nnnn 0000 0000`  | Return the result of a computation              |
| `CALL label arg1 arg2`    | `1000` (0x8)    | `1000 aaaa bbbb 0000`  | Call a function with arguments                  |
| `JUMP addr`               | `1001` (0x9)    | `1001 aaaa aaaa aaaa`  | Jump to an address in memory                    |
| `HALT`                    | `1111` (0xF)    | `1111 0000 0000 0000`  | Stop execution                                  |


Current Idea for asm (rough sketch) {
HVM:
```
@add = {a b} $(([+] a) b)

@main = @add 3 4
```
->
INASM
```asm
@ADD:
    NEW    %PLUS_NODE  "+"      ; Create a "+" node
    LINK   %PLUS_NODE  %A       ; Connect "+" to first argument
    LINK   %PLUS_NODE  %B       ; Connect "+" to second argument
    EVAL   %PLUS_NODE           ; Trigger evaluation
    RETURN %PLUS_NODE           ; Return result

@MAIN:
    NEW    %A        "3"        ; Create node for 3
    NEW    %B        "4"        ; Create node for 4
    CALL   @ADD %A %B           ; Call @ADD with 3 and 4

HALT
```
-> Machine code for PU schedualer : format([ OPCODE (4 bits) ][ DEST (4 bits) ][ SRC1 (4 bits) ][ SRC2 (4 bits) ]
```
BTW HERE I OPTIMIZED AWAY THE CREATION OF THE FUNCITON AND CALLING IT, idk
0001 0011 0000 0011  // NEW %3, value 3
0001 0100 0000 0100  // NEW %4, value 4
0001 0101 0000 1011  // NEW %PLUS_NODE, value "+"
0010 0101 0011 0000  // LINK %PLUS_NODE, %3
0010 0101 0100 0000  // LINK %PLUS_NODE, %4
0101 0101 0000 0000  // EVAL %PLUS_NODE //EXPAND WITH IN todo!()
1111 0000 0000 0000  // HAlt
``` 
or the HEX:
```
0x1303
0x1404
0x150B
0x2530
0x2540
0x5500
0xF
```