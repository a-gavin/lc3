.ORIG x3000         ; this is the address in memory where the program will be loaded
ADD R0, R0, PC      ; Store contents of PC into R0
ADD R0, R0, PC      ; Add offset to PC
JSRR R0             ; Jump to ADD below
HALT
ADD R1, R1, 15      ; Canary instruction, indicates JSRR worked
HALT
.END
