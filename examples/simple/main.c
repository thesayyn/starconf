#include <stdint.h>
#include "config.h"

#ifndef HAVE_CONFIG_H
    #error "Please include config.h first"
#endif

#ifndef HAVE_SOME_HEADER
    #error "Need stddef.h in order to compile"
#endif

#ifndef HAVE_ANOTHER_HEADER
    #error "Need string.h in order to compile"
#endif

int main(void) {
    (void ) sizeof(void*);
}
