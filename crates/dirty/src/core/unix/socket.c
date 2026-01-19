#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

struct SocketResponse {
	int status;
	int server_socket;
};

// the idea is to connect to wayland using sockets
// https://wayland-book.com/protocol-design/interfaces-reqs-events.html
struct SocketResponse create_socket(const char* address)
{
	struct SocketResponse ret = {0};

	int fd = socket(AF_UNIX, SOCK_STREAM, 0);
	if (fd < 0 ) {
		ret.status = -1;
		return ret;
	}

	struct sockaddr_un server_addr = {0};
	server_addr.sun_family = AF_UNIX;
	strncpy(server_addr.sun_path, address, sizeof(server_addr.sun_path) - 1);

	socklen_t len = offsetof(struct sockaddr_un, sun_path)
	                + strlen(server_addr.sun_path) + 1;

	int r = connect(fd, (struct sockaddr*)&server_addr, len);

	ret.server_socket = fd;
	ret.status = r;
	return ret;
}

char* read_socket(int server_socket, char* ch)
{
	read(server_socket, &ch, strlen(ch));
	return ch;
}

void write_socket(int server_socket, char* ch)
	{ write(server_socket, &ch, strlen(ch)); }

void close_socket(int server_socket)
	{ close(server_socket); }
