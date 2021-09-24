        .ORIG x3000         ; this is the address in memory where the program will be loaded
        LEA R0, TRGT        ; store address of TRGT in R0
        JMP R0              ; unconditionally jump to TRGT (stored in R0)
        HALT
TRGT    ADD R1, R1, 1       ; store 1 in R1, canary to indicate LEA/JMP worked
        HALT
        .END
