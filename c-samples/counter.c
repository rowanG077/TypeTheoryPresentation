#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <pthread.h>

void* counter(void* data_ptr) {
	uint64_t* counter = (uint64_t*)data_ptr;

	for (uint64_t i = 0; i < 1000000; ++i) {
		*counter += 1;
	}

	return NULL;
}

int main() {
	pthread_t threads[100];

	uint64_t counter = 0;

	printf("Final counter value: %"PRIu64"\n");
	return 0;
}
