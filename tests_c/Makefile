all: build

build: test

test: test.c
	gcc test.c -o test -I../c_glue -L../target/release -lota_allocator -lpthread -Wl,--no-as-needed -ldl

test_preload: test.c
	LD_PRELOAD=../target/release/libota_allocator.so gcc test.c -o test -I../c_glue -L../target/release -lota_allocator -lpthread

clean:
	rm -f test
