.ORIG x1000                        ; this is the address in memory where the program will be loaded
ADD R0, R0, R1                     ; 0 + 0 and store in R0
ADD R1, R1, 5                      ; 0 + 5 and store in R1
ADD R0, R0, R1                     ; 0 + 5 (from R1) and store in R0
HALT
.END
