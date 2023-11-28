#ifndef ENTRIES_H
#define ENTRIES_H

#include <stdio.h>

char check_operation_type(char* argv[]);
void new_entry(FILE* fptr, char* separator);
void get_time(char* timestamp);
void read_entries(FILE* fptr);
void read_entries_from_start(FILE* fptr, int count);
void read_entries_from_end(FILE* fptr, int count);
int isItEntryHeader(char* line);
int clear_confirmation(void);

#endif /* ENTRIES_H */
