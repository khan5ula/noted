#include "filehandler.h"

#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int build_filepath(char* executablepath, int length, char* filepath) {
  int result = 0;
  int iterator = 0;

  if (readlink("/proc/self/exe", executablepath, length) == -1) {
    perror("Error with the directory path");
    result = 1;
  }

  iterator = get_index_of_src(executablepath);

  if (iterator == 1) {
    perror("Malformatted executable path");
    result = 1;
  }

  if (result != 1) {
    executablepath[iterator] = '\0';
    snprintf(filepath, length + 200, "%s/entries.txt", executablepath);
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
