#include <js_snap.h>

#include <fcntl.h>
#include <pthread.h> 
#include <stddef.h>
#include <stdio.h>
#include <sys/mman.h>
#include <unistd.h>

struct ThreadData {
  pthread_t tid;
  void* data;
  size_t data_length;
};

void* worker_thread(void* vargp) {
  struct ThreadData* thread_data = (struct ThreadData*)(vargp);

  struct JSSnapInstance* instance = js_snap_instance_from_snapshot(
    thread_data->data,
    thread_data->data_length,
    "fns");

  for (int i = 0; i < 10; ++i) {
    const char* result_ptr = NULL;
    int result_len = 0;

  js_snap_instance_call(instance, "Greet", "{}", &result_ptr, &result_len);
    printf(
      "thread %ld, iter %d:\n%.*s\n\n",
      thread_data->tid, i, result_len, result_ptr);
  }

  js_snap_instance_delete(instance);    
  return NULL;
}

int main(int argc, char** argv) {
  if (argc < 2) {
    return 1;
  }

  js_snap_init();

  int fd = open(argv[1], O_RDONLY);
  size_t data_length = lseek(fd, 0, SEEK_END);
  void* data = mmap(0, data_length, PROT_READ, MAP_PRIVATE, fd, 0);

  pthread_t t1;
  pthread_t t2;
  
  struct ThreadData data1 = { t1, data, data_length };
  struct ThreadData data2 = { t1, data, data_length };

  pthread_create(&t1, NULL, worker_thread, &data1);
  pthread_create(&t2, NULL, worker_thread, &data2);

  pthread_join(t1, NULL);
  pthread_join(t2, NULL);

  return 0;
}