#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <sys/mman.h>
#include <errno.h>
#include <ota_allocator.h>

const int N = 2000000000;

void mmap_wrapper(void *addr, int size) {
	int err;
	err = mmap(
		addr,
		size,
		PROT_READ | PROT_WRITE,
		MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED,
		-1,
		0
	);

	printf("err: %d, code: %d\n", err, errno);
}

int main() {
//	ota_init();

	time_t start = time(NULL);

	for (int i = 0; i < N; ++i) {
//		char *s = malloc(/*i % 2048 + */32);
		strcpy(s, "rust rocks");
//		free(s);
//		printf("i: %d\n", i);
	}

	time_t finish = time(NULL);


	printf("Time elapsed: %lds\n", finish - start);

	return 0;
}

//void *addr = 0x00008FFFFFFF0000;
//int size = 4096;
//
//mmap_wrapper(addr, size);
