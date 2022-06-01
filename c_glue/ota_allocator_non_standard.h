#ifndef OTA_ALLOCATOR_NON_STANDARD_H
#define OTA_ALLOCATOR_NON_STANDARD_H

void ota_init(void);

void *ota_malloc(size_t size);

void *ota_calloc(size_t number, size_t size);

void *ota_realloc(void *ptr, size_t size);

void ota_free(void *ptr);

#endif // OTA_ALLOCATOR_NON_STANDARD_H
