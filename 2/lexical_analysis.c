#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <ctype.h>

char keywords[32][10] = {"int", "float", "char", "double", "string"};
char operators[] = "+-*/%=";
char special_symbols[] = ";$";

bool isKeyword(char token[]) {
  for (int i = 0; i < 32; ++i)
    if (strcmp(keywords[i], token) == 0)
      return true;
  return false;
}

bool isSpecialSymbol(char token) {
  for (int i = 0; i < strlen(special_symbols); i++)
    if (token == special_symbols[i])
      return true;
  return false;
}

bool isOperator(char op) {
  for (int i = 0; i < strlen(operators); i++)
    if (op == operators[i])
      return true;
  return false;
}

bool isLiteral(char token[]) {
  if (token[0] == '\'' || token[0] == '"')
    return true;
  return false;
}

bool isConstant(char token[]) {
  char *endptr;
  strtod(token, &endptr);
  if (*endptr == '\0' || (isspace(*endptr) != 0))
    return true;
}

int main() {
  char ch, buffer[15];

  int i, j = 0;

  FILE *fp = fopen("input.txt", "r");

  if (fp == NULL) {
    printf("Error while opening the file\n");
    return 1;
  }

  while ((ch = fgetc(fp)) != EOF) {
    if (isOperator(ch))
      printf("%c is operator\n", ch);

    if (isSpecialSymbol(ch))
      printf("%c is special symbol\n", ch);

    if (isalnum(ch) || ch == '.' || ch == '\'' || ch == '"')
      buffer[j++] = ch;
    else if ((ch == ' ' || ch == '\n') && (j != 0)) {
      buffer[j] = '\0';
      j = 0;

      if (isLiteral(buffer))
        printf("%s is literal\n", buffer);
      else if (isKeyword(buffer))
        printf("%s is keyword\n", buffer);
      else if (isConstant(buffer))
        printf("%s is constant\n", buffer);
      else
        printf("%s is identifier\n", buffer);
    }
  }
  fclose(fp);
  return 0;
}