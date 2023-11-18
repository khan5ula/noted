#include "entries.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#define PRIMARY_COLOR "\033[35m"
#define RESET_COLOR "\033[0m"

int build_filepath(char *executablepath, int exec_length, char *filepath,
                   int file_length) {
  int result = 0;

  if (readlink("/proc/self/exe", executablepath, exec_length) == -1) {
    perror("Error with the directory path");
    result = 1;
  }

  int slashcounter = 0;
  int iterator = strlen(executablepath);

  while (slashcounter < 2) {
    if (executablepath[iterator] == '/')
      slashcounter++;
    iterator--;
  }

  executablepath[++iterator] = '\0';
  snprintf(filepath, file_length, "%s/data/entries.txt", executablepath);

  return result;
}

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

char check_operation_type(char *argv[]) {
  char operation = '.';
  if (argv[1] == NULL) {
    perror("Error: Argument was expected\n");
  }

  if (strcmp(argv[1], "new") == 0 || strcmp(argv[1], "n") == 0) {
    operation = 'a';
  } else if (strcmp(argv[1], "all") == 0 || strcmp(argv[1], "a") == 0) {
    operation = 'r';
  } else if (strcmp(argv[1], "clear") == 0 || strcmp(argv[1], "rm") == 0) {
    operation = 'w';
  } else if (strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "help") == 0) {
    operation = 'h';
  }

  return operation;
}

void new_entry(FILE *fptr, char *separator, int shortstr_len, int max_length) {
  char *entry = malloc(max_length + shortstr_len * 2 + 50);

  if (entry == NULL) {
    perror("Error allocating memory");
    return;
  }

  char *timestamp = entry + max_length;
  char *header = entry + max_length + shortstr_len;

  printf("Post a new entry:\n");
  fgets(entry, max_length, stdin);

  get_time(timestamp);

  if (timestamp[strlen(timestamp) - 1] == '\n')
    timestamp[strlen(timestamp) - 1] = '\0';

  /* Format the header: --- TIMESTAMP --- */
  snprintf(header, shortstr_len, "%s %s %s\n", separator, timestamp, separator);

  fprintf(fptr, "%s%s\n", header, entry);
  printf("Entry saved 📝\n");

  free(entry);
}

void get_time(char *timestamp) {
  time_t time_now = time(NULL);
  snprintf(timestamp, 30, "%s", ctime(&time_now));
}

void read_entries(FILE *fptr, int max_length) {
  char *line = malloc(max_length);
  if (line == NULL) {
    perror("Error allocating memory");
    return;
  }

  while (fgets(line, max_length, fptr) != NULL) {
    printf("%s", line);
  }

  free(line);
}

int clear_confirmation(void) {
  char result = 'N';
  printf("Are you sure you want to clear all entries? [y/N] > ");

  int input;
  while ((input = getchar()) != '\n' && isspace(input))
    ; /* Clear leading whitespace */

  if (input != EOF && input != '\n') {
    result = input;

    while ((input = getchar()) != '\n' && input != EOF)
      ; /* Clear the buffer */
  }

  if (result != 'y') {
    printf("Entries were not cleared\n");
    return 1;
  }

  return 0;
}

void default_action(void) {
  printf("Invalid entries command\n");
  print_options();
}
