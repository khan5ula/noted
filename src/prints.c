#include "prints.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

void print_intro(void) {
  printf("Entries - Quick terminal notes\n\n");
}

void print_options(void) {
  printf("  new");
  printf("\t\t\t\tCreate a new entry (n)\n");
  printf("  all");
  printf("\t\t\t\tRead all entries (a)\n");
  printf("  first");
  printf("\t\t\t\tRead the first entry (f)\n");
  printf("  first COUNT");
  printf("\t\t\tRead a desired number of entries from the start (f COUNT)\n");
  printf("  last");
  printf("\t\t\t\tRead the last entry (l)\n");
  printf("  last COUNT");
  printf("\t\t\tRead a desired number of entries from the end (l COUNT)\n");
  printf("  clear");
  printf("\t\t\t\tDelete all entries (rm)\n\n");
}

void default_action(void) {
  printf("Invalid entries command\n");
  print_options();
}