#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <pthread.h>

static const size_t THREADCOUNT = 20;

// Task to increase the shared counter
void* count(void* data_ptr) {
	uint64_t* counter = (uint64_t*)data_ptr;

	// Increase the counter 1000000 times.
	for (uint64_t i = 0; i < 1000000; ++i) {
		*counter += 1;
	}

	// thread 1       thread 2
	// load counter   load counter
	// counter + 1    counter + 1
	// --------       store counter
	// store counter

	// Wanted : n + 2
	// got:  n + 1

	return NULL;
}

// Simple program that counts to THREADCOUNT * 1000000 using THREADCOUNT
// threads.
int main() {
	// Define an array that contains the thread identifiers.
	pthread_t threads[THREADCOUNT];

	// The counter shared between threads.
	uint64_t counter = 0;

	// Create THREADCOUNT threads and share the counter between them.
	for (size_t i = 0; i < THREADCOUNT; ++i) {
		if (pthread_create(&threads[i], NULL, count, &counter)) {
			perror("Error creating thread.");
			exit(-1);
		}
	}

	// Wait until all threads are finished.
	for (size_t i = 0; i < THREADCOUNT; ++i) {
		pthread_join(threads[i], NULL);
	}

	printf("Final counter value: %"PRIu64"\n", counter);
	return 0;
}
