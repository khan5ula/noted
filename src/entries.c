#include "entries.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

int build_filepath(char* executablepath, int length, char* filepath) {
  int result = 0;
  int iterator = 0;

  if (readlink("/proc/self/exe", executablepath, length) == -1) {
    perror("Error with the directory path");
    result = 1;
  }

  iterator = get_index_of_src(executablepath);

  if (iterator == 0) {
    perror("Malformatted executable path");
    result = 1;
  }

  if (result != 1) {
    executablepath[iterator] = '\0';
    snprintf(filepath, length + 200, "%sdata/entries.txt", executablepath);
  }

  return result;
}

int get_index_of_src(char* path) {
  int indexOfSrc = 1;
  for (int i = strlen(path); i > 0; i--) {
    if (path[i] == 'c') {
      if (path[i - 1] == 'r') {
        if (path[i - 2] == 's') {
          indexOfSrc = i - 2;
          break;
        }
      }
    }
  }
  return indexOfSrc;
}

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
  }

  return operation;
}

/*
Iteration 3:
Post a new entry:
AddressSanitizer:DEADLYSIGNAL
=================================================================
==16914==ERROR: AddressSanitizer: SEGV on unknown address 0x0000000000c0 (pc
0x7f2f4e475078 bp 0x000000000000 sp 0x7ffcb946bf70 T0)
==16914==The signal is caused by a READ memory access.
==16914==Hint: address points to the zero page.
    #0 0x7f2f4e475078 in __vfprintf_internal
stdio-common/vfprintf-internal.c:1218 #1 0x7f2f4e85e88f in
__interceptor_vfprintf
../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors.inc:1664
    #2 0x7f2f4e85e9ce in __interceptor_fprintf
../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors.inc:1721
    #3 0x563225090dc4 in new_entry
(/mnt/void/ohjelmointi/c/entries/src/main+0x2dc4) #4 0x56322509068a in main
(/mnt/void/ohjelmointi/c/entries/src/main+0x268a) #5 0x7f2f4e429d8f in
__libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58 #6
0x7f2f4e429e3f in __libc_start_main_impl ../csu/libc-start.c:392 #7
0x563225090424 in _start (/mnt/void/ohjelmointi/c/entries/src/main+0x2424)
*/

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
