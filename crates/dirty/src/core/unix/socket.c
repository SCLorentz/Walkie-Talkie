#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>

struct SocketResponse {
	int status;
	int server_socket;
};

struct SocketResponse create_socket()
{
	int server_socket;
	struct sockaddr_un server_addr;
	int connection_result;

	server_socket = socket(AF_UNIX, SOCK_STREAM, 0);

	server_addr.sun_family = AF_UNIX;
	strcpy(server_addr.sun_path, "unix_socket");

	connection_result = connect(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr));

	struct SocketResponse socket_return;
	socket_return.server_socket = server_socket;
	socket_return.status = connection_result;

	return socket_return;
}

char read_socket(int server_socket, char ch)
{
	read(server_socket, &ch, 1);
	return ch;
}

void write_socket(int server_socket, char ch)
{
	write(server_socket, &ch, 1);
}

void close_socket(int server_socket)
	{ close(server_socket); }
