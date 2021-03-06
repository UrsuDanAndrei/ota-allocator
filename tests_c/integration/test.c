#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <sys/mman.h>
#include <errno.h>
//#include <ota_allocator.h>

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

//	time_t start = time(NULL);
//	ota_init();
//	for (int i = 0; i < 1; ++i) {
//		char *s = malloc(/*i % 2048 + */32);
//		strcpy(s, "rust rocks");
//		free(s);
////		printf("i: %d\n", i);
//	}
//	char *s = malloc(24);
//	strcpy(s, "vxv");
//	printf("p1: %p, p2: %p\n", s, s + 2);
//	printf("size is: %d\n", malloc_usable_size(s));

//	char *ff = malloc(0);
//	printf("the addr: %p\n", ff);

//	time_t finish = time(NULL);

//	printf("Time elapsed: %lds\n", finish - start);

	char *s1 = malloc(4);
	free(s1);
	char *s2 = malloc(4);
	free(s2);

	printf("p1: %p\n", s1);
	printf("p2: %p\n", s2);

	return 0;
}

// run with RUSTFLAGS="-C target-cpu=native" cargo build --release && make clean && make && ./test
