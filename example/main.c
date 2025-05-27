#include <stdint.h>

#ifndef HAVE_CONFIG_H
    #error "Please include arguments.txt in copts"
#endif

#ifndef __LIBARCHIVE_ENABLE_VISIBILITY
    #error "Please include arguments.txt in copts"
#endif

int main(void) {
    (void ) sizeof(void*);
}
