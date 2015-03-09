CC=gcc
AR=ar
RUSTC=rustc

all: open.a lib
	gcc -shared -o libmylib.so -L . -Wl,--whole-archive liblib.a libopen.a -Wl,--no-whole-archive -ldl -lpthread -lrt -lgcc_s -lpthread -lc -lm

open: src/open_varargs.c
	$(CC) -fPIC -DPIC -c src/open_varargs.c -o open_varargs.o

open.a: open
	$(AR) rcs libopen.a open_varargs.o

lib: src/lib.rs
	$(RUSTC) --crate-type staticlib -C relocation-model=pic src/lib.rs

.PHONY: clean-intermediate
clean-intermediate:
	rm -f liblib.a libopen.a open_varargs.o
.PHONY: clean
clean:
	rm -f liblib.a libopen.a open_varargs.o libmylib.so
