#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <stdatomic.h>

#define N 1000 * 1000

static int shared_int = 0;
// static pthread_mutex_t shared_mutex = PTHREAD_MUTEX_INITIALIZER;

void *increment_a_million_times(void *_) {
  // int err = pthread_mutex_lock(&shared_mutex);
  // assert(err == 0);

  for (int i = 0; i < N; i++) {
    // int err = pthread_mutex_lock(&shared_mutex);
    // assert(err == 0);

    shared_int += 1;

    // atomic_fetch_add(&shared_int, 1);

    // err = pthread_mutex_unlock(&shared_mutex);
    // assert(err == 0);
  }

  // err = pthread_mutex_unlock(&shared_mutex);
  // assert(err == 0);

  return NULL;
}

int main() {
  pthread_t thread1;
  int err = pthread_create(&thread1, NULL, increment_a_million_times, NULL);
  assert(err == 0);

  pthread_t thread2;
  err = pthread_create(&thread2, NULL, increment_a_million_times, NULL);
  assert(err == 0);

  err = pthread_join(thread1, NULL);
  assert(err == 0);

  err = pthread_join(thread2, NULL);
  assert(err == 0);

  printf("expected %d, got %d\n", 2 * N, shared_int);
}
