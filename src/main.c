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

  switch (argc) {
    case 1:
      print_intro();
      print_options();
      break;
    case 2:
      operation = check_operation_type(argv);
      if (operation == 'a') {
        fptr = fopen(filepath, "a");
        if (fptr == NULL) {
          perror("Couldn't handle the file gracefully");
          return 1;
        }
        new_entry(fptr, "---", SHORT_STR_LENGTH, BASIC_LENGTH);
        fclose(fptr);
      } else if (operation == 'r') {
        fptr = fopen(filepath, "r");
        read_entries(fptr, BASIC_LENGTH);
        fclose(fptr);
      } else if (operation == 'w') {
        if (clear_confirmation() == 0) {
          fptr = fopen(filepath, "w");
          printf("Done\n");
          fclose(fptr);
        }
      } else if (operation == 'h') {
        print_intro();
        print_options();
      } else {
        default_action();
        return 1;
      }
      break;
    default:
      default_action();
      return 1;
  }

  return 0;
}