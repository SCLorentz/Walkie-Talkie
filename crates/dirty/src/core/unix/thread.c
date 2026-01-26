#include <pthread.h>

struct c_Thread {
	int id;
	void* thread;
};

struct c_Thread create_thread(typeof(void *(void *)) function)
{
	pthread_t thread;
	int id = pthread_create(&thread, NULL, function, NULL);

	struct c_Thread my_thread;
	my_thread.id = id;
	my_thread.thread = thread;

	return my_thread;
}

void kill_thread(struct c_Thread my_thread)
	{ pthread_cancel(my_thread.thread); }
