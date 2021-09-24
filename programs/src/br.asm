        .ORIG x3000         ; this is the address in memory where the program will be loaded
        BR TRGT             ; branch to TRGT
        ADD R1, R1, 6       ; expected skipped instruction
        HALT
 TRGT   ADD R2, R2, 15      ; store 15 in R1, demonstrating BR worked as intended
        HALT
        .END
