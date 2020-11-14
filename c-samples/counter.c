#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <pthread.h>

static const size_t THREADCOUNT = 20;

void* count(void* data_ptr) {
	uint64_t* counter = (uint64_t*)data_ptr;

	for (uint64_t i = 0; i < 1000000; ++i) {
		*counter += 1;
	}

	return NULL;
}

int main() {
	pthread_t threads[THREADCOUNT];

	uint64_t counter = 0;

	for (size_t i = 0; i < THREADCOUNT; ++i) {
		if (pthread_create(&threads[i], NULL, count, &counter)) {
			perror("Error creating thread.");
			exit(-1);
		}
	}

	for (size_t i = 0; i < THREADCOUNT; ++i) {
		pthread_join(threads[i], NULL);
	}

	printf("Final counter value: %"PRIu64"\n", counter);
	return 0;
}
