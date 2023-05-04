%{
#include<stdio.h>
int operators = 0, special = 0, keywords = 0, identifier = 0, digits = 0;
%}

%%
(int|float|double|string) {keywords++;}
[\-\+\*\/] {operators++;}
[\$\:;\?] {special++;}
[0-9]+ {digits++;}
\s[_A-Za-z][a-zA-Z0-9]+ {identifier++;}
%%

int yywrap() {
return 1;
}
main() {
    printf("Enter String : ");
    yylex();
    printf("keywords %d\n", keywords);
    printf("operators %d\n", operators);
    printf("special %d\n", special);
    printf("digits %d\n", digits);
    printf("identifier %d\n", identifier);
}
