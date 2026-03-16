#include <pthread.h>

struct c_Thread {
	int id;
	void* thread;
};

struct c_Thread create_thread(void *(*function)(void *))
{
	pthread_t thread;
	int id = pthread_create(&thread, NULL, function, NULL);

	struct c_Thread new_thread;
	new_thread.id = id;
	void *ptr = (void *)&thread;
	new_thread.thread = ptr;

	return new_thread;
}

void kill_thread(struct c_Thread thread)
	{ pthread_cancel((pthread_t)thread.thread); }
