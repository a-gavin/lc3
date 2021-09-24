.ORIG x3000                        ; this is the address in memory where the program will be loaded
AND R0, R0, R1                     ; 0 & 0 and store in R0
AND R1, R1, 8                      ; 0 & 8 and store in R1
ADD R0, R0, 15                     ; 0 + 8 and store in R0
AND R2, R0, 1                      ; 8 (from R1) & 1 and store in R0
HALT
.END
