#include "prints.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

void print_intro(void) {
  printf("Entries: Quick terminal notes 📝\n\n");
}

void print_options(void) {
  printf("  new");
  printf("\t\t\tCreate a new entry  (n)\n");
  printf("  all");
  printf("\t\t\tRead all entries (a)\n");
  printf("  clear");
  printf("\t\t\tDelete all entries (rm) RESET_COLOR \n");
}

void default_action(void) {
  printf("Invalid entries command\n");
  print_options();
}