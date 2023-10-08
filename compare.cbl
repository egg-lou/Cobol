      *Compare Number Program
       IDENTIFICATION DIVISION.
       PROGRAM-ID. compare.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 Num1 PIC 9(5).
       01 Num2 PIC 9(5).

       PROCEDURE DIVISION.
           DISPLAY "Enter First Number".
           ACCEPT Num1.
           DISPLAY "Enter Second Number".
           ACCEPT Num2.

           IF Num1 > Num2 THEN
               DISPLAY "The first number is greater"
           ELSE 
               DISPLAY "The second number is greater"
           END-IF.

           STOP RUN.