%{
#include<stdio.h>
int negative = 0, positive = 0;
int negativeFraction = 0, positiveFraction = 0;
%}

%%
[+]?[0-9]+ {positive++;}
[-][0-9]+ {negative++;}
[+]?[0-9]*\.[0-9]+ {positiveFraction++;}
[-][0-9]+\.[0-9]+ {negativeFraction++;}
%%

int yywrap() {
return 1;
}
main() {
    printf("Enter String : ");
    yylex();
    printf("Positive : %d\n", positive);
    printf("Negative : %d\n", negative);
    printf("Positive Fraction : %d\n", positiveFraction);
    printf("Negative Fraction : %d\n", negativeFraction);
}
