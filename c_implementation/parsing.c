#include "mpc.h"
#include <stdio.h>

static char input[2048];

int main(int argc, char **argv) {

  mpc_parser_t *Adjective = mpc_new("adjective");
  mpc_parser_t *Noun = mpc_new("noun");
  mpc_parser_t *Phrase = mpc_new("phrase");
  mpc_parser_t *Doge = mpc_new("doge");

  mpca_lang(MPCA_LANG_DEFAULT, "                                           \
    adjective : \"wow\" | \"many\"            \
              |  \"so\" | \"such\";           \
    noun      : \"lisp\" | \"language\"       \
              | \"book\" | \"build\" | \"c\"; \
    phrase    : <adjective> <noun>;           \
    doge      : <phrase>*;                    \
  ",
            Adjective, Noun, Phrase, Doge);

  /* Do some parsing here... */

  mpc_cleanup(4, Adjective, Noun, Phrase, Doge);
  puts("Anotlisp version 0.0.0.1");
  puts("Press Ctrl+c to Exit \n");

  while (1) {
    fputs("Anotlisp> ", stdout);

    fgets(input, 2048, stdin);

    printf("No you're a %s", input);
  }
  return 0;
}
