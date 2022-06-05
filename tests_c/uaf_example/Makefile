LIB_LOCATION=$(pwd)/../target/release

all: build

build: uaf uaf_debug uaf_regular

uaf: uaf.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release \
	gcc uaf.c -o uaf \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue  \
		-L/home/dan/workspace/licenta/ota-allocator/target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

uaf_debug: uaf.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug \
	gcc uaf.c -o uaf_debug \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/target/debug \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

uaf_regular: uaf.c
	gcc uaf.c -o uaf_regular -I/home/dan/workspace/licenta/ota-allocator/c_glue

clean:
	rm -f uaf uaf_debug uaf_regular
