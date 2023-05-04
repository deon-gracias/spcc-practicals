%{
#include<stdio.h>
int chars = 0, spaces = 0, endOfLines = 0, words = 0;
%}

%%
[ ] {spaces++;}
[.] {chars++;}
[a-zA-Z]{2,} {words++;}
[\n] {endOfLines++;}
%%

int yywrap() {
return 1;
}
main() {
    printf("Enter String : ");
    yylex();
    printf("Chars : %d\n", chars);
    printf("Spaces : %d\n", spaces);
    printf("End of Lines : %d\n", endOfLines);
    printf("Words : %d\n", words);
}
