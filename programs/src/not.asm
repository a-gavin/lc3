.ORIG x3000                        ; this is the address in memory where the program will be loaded
NOT R0, R0                         ; not 0 and store in R0
NOT R0, R0                         ; not 0 and store in R0
ADD R1, R1, 8                      ; 0 + 8 and store in R1
NOT R1, R1                         ; not 8 and store in R1
HALT
.END
