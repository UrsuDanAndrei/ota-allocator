#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define RESV_THREADS_NO 0

#define RESV_ADDRS_NO 0

#define PAGE_SIZE 4096

#define FIRST_ADDR_SPACE_START 206158430208

#define ADDR_SPACE_MAX_SIZE 68719476736

#define ADDR_SPACE_MASK 18446744004990074880ULL

#define LARGE_ADDR_SPACE_OFFSET 34359738368

#define LARGE_ADDR_SPACE_MASK 34359738368

#define META_ADDR_SPACE_START 137438953472

#define META_ADDR_SPACE_MAX_SIZE (32 * PAGE_SIZE)

#define POOL_SIZE (2 * PAGE_SIZE)

#define TANK_SIZE (512 * POOL_SIZE)

#define STANDARD_ALIGN 16

#define BINS_NO 10

#define TEST_ADDR_SPACE_START 68719476736

#define TEST_ADDR_SPACE_MAX_SIZE (32 * PAGE_SIZE)

void ota_init(void);

uint8_t *ota_malloc(uintptr_t size);

void ota_free(uint8_t *addr);
