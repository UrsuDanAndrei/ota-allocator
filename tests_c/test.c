#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ota_allocator.h>

int main() {
	ota_init();

	char *s = ota_malloc(32);
	strcpy(s, "rust rocks");

	printf("Hello world, %s!\n", s);
	ota_free(s);

	return 0;
}
