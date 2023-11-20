#ifndef ENTRIES_H
#define ENTRIES_H

#include <stdio.h>

int build_filepath(char* executablepath, int length, char* filepath);
int get_index_of_src(char* path);
char check_operation_type(char* argv[]);
void new_entry(FILE* fptr, char* separator, int shortstr_len, int max_length);
void get_time(char* timestamp);
void read_entries(FILE* fptr, int max_length);
int clear_confirmation(void);

#endif /* ENTRIES_H */
