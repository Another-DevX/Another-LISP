#include "mpc.h"
#include <stdio.h>

static char input[2048];

int main(int argc, char **argv) {

  mpc_parser_t *Number = mpc_new("number");
  mpc_parser_t *Operator = mpc_new("operator");
  mpc_parser_t *Expression = mpc_new("expression");
  mpc_parser_t *Anotlisp = mpc_new("anotlisp");

  mpca_lang(MPCA_LANG_DEFAULT, "                                           \
    number       : /-?[0-9]+/;                                      \
    operator     :  '+' | '-' | '*' | '/';                          \
    expression   :  <number> | '(' <operator> <expression>+ ')';    \
    anotlisp      : /^/ <operator> <expression>+ /$/ ;              \
  ",
            Number, Operator, Expression, Anotlisp);

  puts("Anotlisp version 0.0.0.1");
  puts("Press Ctrl+c to Exit \n");

  while (1) {
    fputs("Anotlisp> ", stdout);

    fgets(input, 2048, stdin);

    mpc_result_t r;
    if (mpc_parse("<stdin>", input, Anotlisp, &r)) {
      mpc_ast_print(r.output);
      mpc_ast_delete(r.output);
    } else {
      mpc_err_print(r.error);
      mpc_err_delete(r.error);
    }
  }

  mpc_cleanup(4, Number, Operator, Expression, Anotlisp);
  return 0;
}
