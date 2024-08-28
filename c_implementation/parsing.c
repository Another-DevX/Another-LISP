#include "mpc.h"
#include <stdio.h>

static char input[2048];

int number_of_nodes(mpc_ast_t *t) {
  if (t->children_num == 0) {
    return 1;
  }
  if (t->children_num >= 1) {
    int total = 1;
    for (int i = 0; i < t->children_num; i++) {
      total += number_of_nodes(t->children[i]);
    }
    return total;
  }
  return 0;
}

long eval_op(long x, char *op, long y) {
  if (strcmp(op, "+") == 0)
    return x + y;
  if (strcmp(op, "-") == 0)
    return x - y;
  if (strcmp(op, "*") == 0)
    return x * y;
  if (strcmp(op, "/") == 0)
    return x / y;
  if (strcmp(op, "%") == 0)
    return x % y;
  return 0;
}

long eval(mpc_ast_t *t) {
  if (strstr(t->tag, "number")) {
    return atoi(t->contents);
  }
  char *op = t->children[1]->contents;
  long x = eval(t->children[2]);
  int i = 3;
  while (strstr(t->children[i]->tag, "expression")) {
    x = eval_op(x, op, eval(t->children[i]));
    i++;
  }
  return x;
}

int main(int argc, char **argv) {

  mpc_parser_t *Number = mpc_new("number");
  mpc_parser_t *Operator = mpc_new("operator");
  mpc_parser_t *Expression = mpc_new("expression");
  mpc_parser_t *Anotlisp = mpc_new("anotlisp");

  mpca_lang(MPCA_LANG_DEFAULT, "                                    \
    number       : /-?[0-9]+/;                                      \
    operator     :  '+' | '-' | '*' | '/' | '%' ;                   \
    expression   :  <number> | '(' <operator> <expression>+ ')';    \
    anotlisp     : /^/ <operator> <expression>+ /$/ ;              \
  ",
            Number, Operator, Expression, Anotlisp);

  puts("Anotlisp version 0.0.0.1");
  puts("Press Ctrl+c to Exit \n");

  while (1) {
    fputs("Anotlisp> ", stdout);

    fgets(input, 2048, stdin);

    mpc_result_t r;
    if (mpc_parse("<stdin>", input, Anotlisp, &r)) {
      long result = eval(r.output);
      printf("%li\n", result);
      mpc_ast_delete(r.output);
    } else {
      mpc_err_print(r.error);
      mpc_err_delete(r.error);
    }
  }

  mpc_cleanup(4, Number, Operator, Expression, Anotlisp);
  return 0;
}
