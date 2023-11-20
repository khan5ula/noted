#include "prints.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

void print_intro(void) {
  printf(PRIMARY_COLOR "Entries" RESET_COLOR ": Quick terminal notes 📝\n\n");
}

void print_options(void) {
  printf("  " PRIMARY_COLOR "new");
  printf(RESET_COLOR "\t\t\t\tCreate a new entry " PRIMARY_COLOR "(n)\n");
  printf("  " PRIMARY_COLOR "all");
  printf(RESET_COLOR "\t\t\t\tRead all entries " PRIMARY_COLOR "(a)\n");
  printf("  " PRIMARY_COLOR "clear");
  printf(RESET_COLOR "\t\t\t\tDelete all entries " PRIMARY_COLOR
                     "(rm)" RESET_COLOR "\n");
}

void default_action(void) {
  printf("Invalid entries command\n");
  print_options();
}