LIB_LOCATION=$(pwd)/../target/release

all: build

build: test

test: test.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/username/foo:/home/dan/workspace/licenta/ota-allocator/tests_c/../target/release
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/tests_c/../target/release \
	gcc test.c -o test \
		-I../c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/tests_c/../target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

test_debug: test.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH::/home/dan/workspace/licenta/ota-allocator/tests_c/../target/debug
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/tests_c/../target/debug \
	gcc test.c -o test \
		-I../c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/tests_c/../target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

test_regular: test.c
	gcc test.c -o test -I../c_glue

clean:
	rm -f test
