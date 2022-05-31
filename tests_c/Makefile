all: build

build: test

test: test.c
	gcc test.c -o test -I../c_glue -L../target/release -lota_allocator -lpthread -Wl,--no-as-needed -ldl

clean:
	rm -f test
