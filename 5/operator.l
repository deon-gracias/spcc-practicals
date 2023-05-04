%{
#include<stdio.h>
int operators = 0, operands = 0;
%}

%%
[\-\+\*\/] {operators++;}
[A-Za-z0-9]+ {operands++;}
%%

int yywrap() {
return 1;
}
main() {
    printf("Enter String : ");
    yylex();
    printf("Operators : %d\n", operators);
    printf("Operands : %d\n", operands);
}
