#ifndef OTA_ALLOCATOR_H
#define OTA_ALLOCATOR_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void ota_init(void);

void *malloc(size_t size);

void *calloc(size_t number, size_t size);

void *realloc(void *ptr, size_t size);

void free(void *ptr);

size_t malloc_usable_size(const void *ptr);

#endif // OTA_ALLOCATOR_H
