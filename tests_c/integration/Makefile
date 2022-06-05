LIB_LOCATION=$(pwd)/../target/release

all: build

build: test test_debug test_regular

test: test.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release \
	gcc test.c -o test \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue  \
		-L/home/dan/workspace/licenta/ota-allocator/target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

test_debug: test.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug \
	gcc test.c -o test_debug \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/target/debug \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

test_regular: test.c
	gcc test.c -o test_regular -I/home/dan/workspace/licenta/ota-allocator/target/debug

clean:
	rm -f test
