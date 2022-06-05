LIB_LOCATION=$(pwd)/../target/release

all: build

build: baby_heap baby_heap_debug baby_heap_regular

baby_heap: baby_heap.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release \
	gcc baby_heap.c -o baby_heap \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue  \
		-L/home/dan/workspace/licenta/ota-allocator/target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

baby_heap_debug: baby_heap.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug \
	gcc baby_heap.c -o baby_heap_debug \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/target/debug \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

baby_heap_regular: baby_heap.c
	gcc baby_heap.c -o baby_heap_regular -I/home/dan/workspace/licenta/ota-allocator/c_glue

clean:
	rm -f baby_heap baby_heap_debug baby_heap_regular
