#include "entries.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#define HEADER_LENGTH 61
#define TIMESTAMP_LENGTH 51
#define CONTENT_LENGTH 1001
#define LINE char line[CONTENT_LENGTH] = "\0"

struct entry {
  char header[HEADER_LENGTH];
  char content[CONTENT_LENGTH];
};

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

void new_entry(FILE* fptr, char* separator) {
  char entry[CONTENT_LENGTH] = "\0";
  char timestamp[TIMESTAMP_LENGTH] = "\0";
  char header[HEADER_LENGTH] = "\0";

  printf("Post a new entry:\n");
  fgets(entry, CONTENT_LENGTH, stdin);

  get_time(timestamp);

  if (timestamp[strlen(timestamp) - 1] == '\n')
    timestamp[strlen(timestamp) - 1] = '\0';

  if (snprintf(header, HEADER_LENGTH, "%s %s %s\n", separator, timestamp,
               separator) >= TIMESTAMP_LENGTH) {
    perror("Error while creating entry header");
  }

  fprintf(fptr, "%s%s\n", header, entry);
  printf("Entry saved 📝\n");
}

void get_time(char* timestamp) {
  time_t time_now = time(NULL);
  if (snprintf(timestamp, TIMESTAMP_LENGTH, "%s", ctime(&time_now)) >=
      TIMESTAMP_LENGTH) {
    perror("Error while creating timestamp");
  }
}

void read_entries(FILE* fptr) {
  LINE;
  while (fgets(line, CONTENT_LENGTH, fptr) != NULL)
    printf("%s", line);
}

void read_entries_from_start(FILE* fptr, int count) {
  LINE;
  int endOfEntryCount = 0;

  while (fgets(line, CONTENT_LENGTH, fptr) != NULL &&
         endOfEntryCount < (count + 1)) {
    if (isItEntryHeader(line) == 0)
      endOfEntryCount++;

    if (endOfEntryCount <= count)
      printf("%s", line);
  }
}

void read_entries_from_end(FILE* fptr, int count) {
  LINE;
  int noOfEntries = 0;
  int endOfEntryCount = 0;

  while (fgets(line, CONTENT_LENGTH, fptr) != NULL) {
    if (isItEntryHeader(line) == 0) {
      noOfEntries++;
    }
  }

  rewind(fptr);

  struct entry* entries = malloc(noOfEntries * sizeof(struct entry));

  if (entries == NULL) {
    perror("Error allocating memory");
    free(entries);
    return;
  }

  while (fgets(line, CONTENT_LENGTH, fptr) != NULL) {
    if (isItEntryHeader(line) == 0) {
      strcpy(entries[endOfEntryCount++].header, line);
    } else {
      strcat(entries[endOfEntryCount - 1].content, line);
    }
  }

  if (noOfEntries - count < 0)
    count = 1;

  for (int i = noOfEntries - count; i < noOfEntries; i++) {
    printf("%s%s", entries[i].header, entries[i].content);
  }

  free(entries);
}

int isItEntryHeader(char* line) {
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