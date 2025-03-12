# HIN
Hardware Interaction Net (HIN), evaluating Interaction nets in parallel on a new, custom PU


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