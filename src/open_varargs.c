#include <fcntl.h>
#include <sys/stat.h>
#include <stdarg.h>
#include <stdio.h>

/* functions to be provided in Rust */
int open_create(const char *pathname, int flags, mode_t m);
int open_nocreate(const char *pathname, int flags);

/*int open_create(const char *pathname, int flags, mode_t mode) {
    printf("whatevski man\n");
    return 3;
}

int open_nocreate(const char *pathname, int flags) {
    printf("whatever man\n");
    return 4;
}*/

int open(const char *pathname, int flags, ...) {
    mode_t m = 0;
    if(flags & O_CREAT) {
        va_list a;
        va_start(a, flags);
        m = va_arg(a, mode_t);
        va_end(a);

        return open_create(pathname, flags, m);
    }
    else {
        return open_nocreate(pathname, flags);
    }
}

