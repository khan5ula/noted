#include "entries.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

char check_operation_type(char* argv[]) {
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
  } else if (strcmp(argv[1], "last") == 0 || strcmp(argv[1], "l") == 0) {
    operation = 'l';
  } else if (strcmp(argv[1], "first") == 0 || strcmp(argv[1], "f") == 0) {
    operation = 'f';
  }

  return operation;
}

void new_entry(FILE* fptr, char* separator, int shortstr_len, int max_length) {
  char* entry = malloc(max_length + shortstr_len * 2 + 50);

  if (entry == NULL) {
    free(entry);
    perror("Error allocating memory");
    return;
  }

  char* timestamp = entry + max_length;
  char* header = entry + max_length + shortstr_len;

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

void get_time(char* timestamp) {
  time_t time_now = time(NULL);
  snprintf(timestamp, 30, "%s", ctime(&time_now));
}

void read_entries(FILE* fptr, int max_length) {
  char* line = malloc(max_length);
  if (line == NULL) {
    perror("Error allocating memory");
    free(line);
    return;
  }

  while (fgets(line, max_length, fptr) != NULL) {
    printf("%s", line);
  }

  free(line);
}

void read_entries_from_start(FILE* fptr, int max_length, int count) {
  char* line = malloc(max_length);
  int endOfEntryCount = 0;
  count++;

  if (line == NULL) {
    perror("Error allocating memory");
    free(line);
    return;
  }

  while (fgets(line, max_length, fptr) != NULL && endOfEntryCount <= count) {
    if (checkForEndOfEntry(line) == 0)
      endOfEntryCount++;

    if (endOfEntryCount < count)
      printf("%s", line);
  }

  free(line);
}

int checkForEndOfEntry(char* line) {
  int result = 0;
  int length = strlen(line);

  if (length < 25 || length > 40) {
    result = 1;
  } else {
    for (int i = 0, j = length - 2; i < 3; i++, j--) {
      if (line[i] != '-' || line[j] != '-') {
        result = 1;
        break;
      }
    }
  }

  return result;
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