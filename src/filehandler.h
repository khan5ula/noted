#ifndef FILEHANDLER_H
#define FILEHANDLER_H

#include <stdio.h>

int build_filepath(char* executablepath, int length, char* filepath);
int get_index_of_src(char* path);
int getCountOfDsrdEntries(int argc, char* argv[]);

#endif