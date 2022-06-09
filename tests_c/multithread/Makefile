LIB_LOCATION=$(pwd)/../target/release

all: build

build: multithread multithread_debug multithread_regular

multithread: multithread.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release \
	gcc multithread.c -o multithread \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue  \
		-L/home/dan/workspace/licenta/ota-allocator/target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

multithread_debug: multithread.c
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug \
	gcc multithread.c -o multithread_debug \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/target/debug \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

multithread_regular: multithread.c
	gcc multithread.c -o multithread_regular -I/home/dan/workspace/licenta/ota-allocator/c_glue -lpthread

clean:
	rm -f multithread multithread_debug multithread_regular
