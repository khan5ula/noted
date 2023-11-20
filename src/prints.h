#ifndef PRINTS_H
#define PRINTS_H

#include <stdio.h>

#define PRIMARY_COLOR "\033[35m"
#define RESET_COLOR "\033[0m"

void print_intro(void);
void print_options(void);
void default_action(void);

#endif /* PRINTS_H */