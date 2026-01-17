#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>

struct SocketResponse {
	int status;
	char response;
};

struct SocketResponse create_socket()
{
	int server_socket;
	struct sockaddr_un server_addr;
	struct SocketResponse r;
	int connection_result;

	char ch='C';

	server_socket = socket(AF_UNIX, SOCK_STREAM, 0);

	server_addr.sun_family = AF_UNIX;
	strcpy(server_addr.sun_path, "unix_socket");

	connection_result = connect(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr));

	if (connection_result == -1) {
		r.status = 1;
		return r;
	}

	write(server_socket, &ch, 1);
	read(server_socket, &ch, 1);
	close(server_socket);

	r.status = 1;
	r.response = ch;

	return r;
}
