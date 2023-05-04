#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

char exp[100];
int i = 0;

void E_dash(), T_dash(), E(), T(), F();
bool check_grammer();

int main() {
    printf("Enter an expression : ");
    fgets(exp, sizeof(exp), stdin);
    check_grammer();

    return 0;
}

bool check_grammer() {
    int n = strlen(exp);
    // printf("%s %d: ", exp, n);

    E(exp);
    if (i == n - 1) {
        printf("Input Accepted\n");
        return true;
    }

    printf("Input Rejected\n");
    return false;
}

void advance() {
    i++;
}

void E_dash() {
    if (exp[i] == '+') {
        advance();
        T();
        E_dash();
    }
}

void T() {
    F();
    T_dash();
}

void T_dash() {
    if (exp[i] == '*') {
        advance();
        F();
        T_dash();
    }
}

void E() {
    T();
    E_dash();
}

void F() {
    if (exp[i] == '(') {
        advance();
        E();
        if (exp[i] == ')') {
            advance();
            return;
        }
    } else if(exp[i] == 'i') {
        advance();
        return;
    }

    printf("ERROR\n");
    exit(0);
    return;
}