#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <semaphore.h>
#include <unistd.h>

long long n;

void* thread_fn(void* arg)
{
	for (int i = 0; i < n; ++i) {
		char *x = malloc(32);
		free(x);
	}
}

int main(int argc, char *argv[])
{
    pthread_t tid[32];
	n = atoll(argv[1]);
    int p = atoi(argv[2]);

	printf("n: %lld, p: %d\n", n, p);

	time_t start = time(NULL);

	for (int i = 0; i < p; ++i) {
		pthread_create(&tid[i], NULL, thread_fn, NULL);
	}

    for (int i = 0; i < p; ++i) {
		pthread_join(tid[i], NULL);
	}

	time_t finish = time(NULL);

	printf("Time elapsed: %lds\n", finish - start);

    return 0;
}
