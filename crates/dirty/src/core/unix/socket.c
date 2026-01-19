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
struct SocketResponse create_socket(char* address)
{
	int server_socket;
	struct sockaddr_un server_addr;
	int connection_result;

	server_socket = socket(AF_UNIX, SOCK_STREAM, 0);

	server_addr.sun_family = AF_UNIX;
	strcpy(server_addr.sun_path, address);

	connection_result = connect(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr));

	struct SocketResponse socket_return;
	socket_return.server_socket = server_socket;
	socket_return.status = connection_result;

	return socket_return;
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
