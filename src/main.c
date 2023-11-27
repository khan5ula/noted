/* main.c
 *
 * Copyright 2023 Kristian Hannula
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include <inttypes.h>
#include "entries.h"
#include "filehandler.h"
#include "prints.h"

#define SHORT_STR_LENGTH 60
#define BASIC_LENGTH 600

int main(int argc, char* argv[]) {
  FILE* fptr;
  char operation = '.';
  char executablepath[BASIC_LENGTH];
  char filepath[BASIC_LENGTH + 200];

  if (build_filepath(executablepath, BASIC_LENGTH, filepath) == 1) {
    return 1;
  }

  if (argc == 1) {
    print_intro();
    print_options();
    return 0;
  }

  operation = check_operation_type(argv);

  if (operation == 'a') {
    fptr = fopen(filepath, "a");
    if (fptr == NULL) {
      perror("Couldn't open or create the file");
      return 1;
    }
    new_entry(fptr, "---", SHORT_STR_LENGTH, BASIC_LENGTH);
    fclose(fptr);
  } else if (operation == 'r') {
    fptr = fopen(filepath, "r");
    if (fptr == NULL) {
      perror("Couldn't open the file");
      return 1;
    }
    read_entries(fptr, BASIC_LENGTH);
    fclose(fptr);
  } else if (operation == 'w') {
    if (clear_confirmation() == 0) {
      fptr = fopen(filepath, "w");
      if (fptr == NULL) {
        perror("Something went wrong when processing the file");
        return 1;
      }
      printf("Done\n");
      fclose(fptr);
    }
  } else if (operation == 'h') {
    print_intro();
    print_options();
  } else if (operation == 'l') {
    fptr = fopen(filepath, "r");
    if (fptr == NULL) {
      perror("Couldn't open the file");
      return 1;
    }
    // call the function to read the last entry
    fclose(fptr);
  } else if (operation == 'f') {
    int countOfDsrdEntries = 1;
    char* endptr;

    fptr = fopen(filepath, "r");
    if (fptr == NULL) {
      perror("Couldn't open the file");
      return 1;
    }

    if (argc > 2)
      countOfDsrdEntries = strtoimax(argv[2], &endptr, 10);

    read_entries_from_start(fptr, BASIC_LENGTH, countOfDsrdEntries);
    fclose(fptr);
  } else {
    default_action();
    return 1;
  }

  return 0;
}