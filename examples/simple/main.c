#include <stdint.h>

#ifndef HAVE_CONFIG_H
    #error "Please include config.h first"
#endif

#ifndef HAVE_SOME_HEADER
    #error "Sorry can't compile"
#endif

int main(void) {
    (void ) sizeof(void*);
}
