#ifndef ENTRIES_H
#define ENTRIES_H

#include <stdio.h>

int build_filepath(char *executablepath, int exec_length, char *filepath,
                   int file_length);
void print_intro(void);
void print_options(void);
char check_operation_type(char *argv[]);
void new_entry(FILE *fptr, char *separator, int shortstr_len, int max_length);
void get_time(char *timestamp);
void read_entries(FILE *fptr, int max_length);
int clear_confirmation(void);
void default_action(void);

#endif /* ENTRIES_H */
