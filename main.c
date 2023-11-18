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

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <unistd.h>

void print_intro(void);
void print_options(void);
char check_operation_type(char *argv[]);
void new_entry(FILE *fptr, char *separator);
void get_time(char *timestamp);
void format_header(char *header, char *separator, char *timestamp);
void read_entries(FILE *fptr);
void clear_entries(FILE *fptr);
int clear_confirmation(void);
void default_action(void);

int main (int argc, char *argv[]) {
	FILE *fptr;
	char operation = '.';
	char executablePath[900];
	char filepath[1024];
	int iterator = 0;
	
	if (readlink("/proc/self/exe", executablePath, sizeof(executablePath)) == -1) {
		perror("Error with the directory path");
		return 1;
	}
	
	for (iterator = strlen(executablePath); executablePath[iterator] != '/'; iterator--);
	executablePath[++iterator] = '\0';
	snprintf(filepath, sizeof(filepath), "%s/entries.txt", executablePath);

	switch(argc) {
		case 1:
			print_intro();
			print_options();
			break;
		case 2:
			operation = check_operation_type(argv);
			if (operation == 'a') {
				fptr = fopen(filepath, "a");
				new_entry(fptr, "---");
			} else if (operation == 'r') {
				fptr = fopen(filepath, "r");
				read_entries(fptr);
			} else if (operation == 'w') {
				if (clear_confirmation() == 0)  {
					fptr = fopen(filepath, "w");
					clear_entries(fptr);
				}
			} else {
				default_action();
			  return 1;
			}
			fclose(fptr);
			break;
		default:
			default_action();
			return 1;
	}

	return 0;
}

void print_intro(void) {
	printf("make quick terminal notes with \033[35mentries 📝\n\033[0m");
	printf("- - - - - - - - - - - - - - - - - - - - -\n");
}

void print_options(void) {
	printf("entries \033[35mwrite");
	printf("\033[0m\t\tcreate a new entry\n");
	printf("entries \033[35mread");
	printf("\033[0m\t\tread all entries\n");
}

char check_operation_type(char *argv[]) {
	char operation = '.';
	if (argv[1] == NULL) {
		perror("Error: Argument was expected\n");
	}

	if (strcmp(argv[1], "write") == 0) {
		operation = 'a';
	} else if (strcmp(argv[1], "read") == 0) {
		operation = 'r';
	} else if (strcmp(argv[1], "clear") == 0) {
		operation = 'w';
	}

	return operation;
}

void new_entry(FILE *fptr, char *separator) {
	char entry[512] = {'\0'};
	char timestamp[50] = {'\0'};
	char header[40] = {'\0'};

	printf("\033[35mnew entry:\n\033[0m");
	fgets (entry, 512, stdin);

	get_time(timestamp);

	if (timestamp[strlen(timestamp)-1] == '\n')
		timestamp[strlen(timestamp)-1] = '\0';

	format_header(header, separator, timestamp);

	fprintf(fptr, "%s%s\n", header, entry);
	printf("\nentry saved 📝\n");
}

void get_time(char *timestamp) {
	time_t time_now = time(NULL);
	snprintf(timestamp, 30, "%s", ctime(&time_now));
}

void format_header(char *header, char *separator, char *timestamp) {
	snprintf(header, 50, "%s %s %s\n", separator, timestamp, separator);
}

void read_entries(FILE *fptr) {
	char line[512] = {"\0"};
	while (fgets(line, 512, fptr) != NULL) {
		printf("%s", line);
	}
}

void clear_entries(FILE *fptr) {
	if (ftruncate(fileno(fptr), 0) == -1) {
        perror("Error clearing entries.");
    } else {
		printf("Done.\n");
	}
}

int clear_confirmation(void) {
	char result = 'N';
	printf("Are you sure you want to clear all entries? [y/N] > ");
	scanf("%c", &result);
	
	if (result != 'y') {
		printf("Entries were not cleared.\n");
		return 1;
	}
	
	return 0;
}

void default_action(void) {
	printf("Invalid Entries command.\n");
	print_options();	
}

