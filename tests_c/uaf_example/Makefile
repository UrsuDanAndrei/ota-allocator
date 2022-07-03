OTA_LIB_DIR=/home/dan/workspace/licenta/ota-allocator/tests_c/../target/release
OTA_LIB_PATH=$(OTA_LIB_DIR)/libota_allocator.so

all: build

build: uaf_ota uaf_libc

uaf_ota: uaf.c
	LD_PRELOAD=$(OTA_LIB_PATH) LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):$(OTA_LIB_DIR) \
		gcc uaf.c -o uaf_ota -Wno-format -lpthread \
		-L$(OTA_LIB_DIR) -lota_allocator

uaf_libc: uaf.c
	gcc uaf.c -o uaf_libc -Wno-format -lpthread

clean:
	rm -f uaf_ota uaf_libc
