#ifndef ENTRIES_H
#define ENTRIES_H

#include <stdio.h>

char check_operation_type(char* argv[]);
void new_entry(FILE* fptr, char* separator, int shortstr_len, int max_length);
void get_time(char* timestamp);
void read_entries(FILE* fptr, int max_length);
void read_entries_from_start(FILE* fptr, int max_length, int count);
int checkForEndOfEntry(char* line);
int clear_confirmation(void);

#endif /* ENTRIES_H */
