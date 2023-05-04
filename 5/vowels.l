%{
#include<stdio.h>
int vow = 0, cons = 0;
%}
%%
[aeiou] {vow++;}
[a-zA-Z] {cons++;}
%%
int yywrap() {
return 1;
}
main() {
    printf("Enter String : ");
    yylex();
    printf("No of vowels : %d\n", vow);
    printf("No of consonants : %d\n", cons);
}
