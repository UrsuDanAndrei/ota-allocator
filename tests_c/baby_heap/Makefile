# baby_heap.c Makefile

OTA_LIB_DIR=/home/dan/workspace/licenta/ota-allocator/tests_c/../target/release
OTA_LIB_PATH=$(OTA_LIB_DIR)/libota_allocator.so

all: build

build: baby_heap_ota baby_heap_libc

baby_heap_ota: baby_heap.c
	LD_PRELOAD=$(OTA_LIB_PATH) LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):$(OTA_LIB_DIR) \
		gcc baby_heap.c -o baby_heap_ota -w -lpthread \
		-L$(OTA_LIB_DIR) -lota_allocator

baby_heap_libc: baby_heap.c
	gcc baby_heap.c -o baby_heap_libc -w -lpthread

clean:
	rm -f baby_heap_ota baby_heap_libc
