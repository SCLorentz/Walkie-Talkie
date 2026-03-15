// https://gaultier.github.io/blog/wayland_from_scratch.html

#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <bits/stdint-uintn.h>
#include <stdlib.h>
#include <stdio.h>

#define cstring_len(s) (sizeof(s) - 1)
#define roundup_4(n) (((n) + 3) & -4)

static uint32_t wayland_current_id = 1;

static const uint32_t wayland_display_object_id = 1;
static const uint16_t wayland_wl_registry_event_global = 0;
static const uint16_t wayland_shm_pool_event_format = 0;
static const uint16_t wayland_wl_buffer_event_release = 0;
static const uint16_t wayland_xdg_wm_base_event_ping = 0;
static const uint16_t wayland_xdg_toplevel_event_configure = 0;
static const uint16_t wayland_xdg_toplevel_event_close = 1;
static const uint16_t wayland_xdg_surface_event_configure = 0;
static const uint16_t wayland_wl_display_get_registry_opcode = 1;
static const uint16_t wayland_wl_registry_bind_opcode = 0;
static const uint16_t wayland_wl_compositor_create_surface_opcode = 0;
static const uint16_t wayland_xdg_wm_base_pong_opcode = 3;
static const uint16_t wayland_xdg_surface_ack_configure_opcode = 4;
static const uint16_t wayland_wl_shm_create_pool_opcode = 0;
static const uint16_t wayland_xdg_wm_base_get_xdg_surface_opcode = 2;
static const uint16_t wayland_wl_shm_pool_create_buffer_opcode = 0;
static const uint16_t wayland_wl_surface_attach_opcode = 1;
static const uint16_t wayland_xdg_surface_get_toplevel_opcode = 1;
static const uint16_t wayland_wl_surface_commit_opcode = 6;
static const uint16_t wayland_wl_display_error_event = 0;
static const uint32_t wayland_format_xrgb8888 = 1;
static const uint32_t wayland_header_size = 8;
static const uint32_t color_channels = 4;

typedef enum state_state_t state_state_t;
enum state_state_t {
    STATE_NONE,
    STATE_SURFACE_ACKED_CONFIGURE,
    STATE_SURFACE_ATTACHED,
};

typedef struct state_t state_t;
struct state_t {
    uint32_t wl_registry;
    uint32_t wl_shm;
    uint32_t wl_shm_pool;
    uint32_t wl_buffer;
    uint32_t xdg_wm_base;
    uint32_t xdg_surface;
    uint32_t wl_compositor;
    uint32_t wl_surface;
    uint32_t xdg_toplevel;
    uint32_t stride;
    uint32_t w;
    uint32_t h;
    uint32_t shm_pool_size;
    int shm_fd;
    uint8_t *shm_pool_data;

    state_state_t state;
};

static void buf_write_u32(char *buf, uint64_t *buf_size, uint64_t buf_cap, uint32_t x) {
    *(uint32_t *)(buf + *buf_size) = x;
    *buf_size += sizeof(x);
}

static void buf_write_u16(char *buf, uint64_t *buf_size, uint64_t buf_cap, uint16_t x) {
    *(uint16_t *)(buf + *buf_size) = x;
    *buf_size += sizeof(x);
}

static void buf_write_string(char *buf, uint64_t *buf_size, uint64_t buf_cap,
    char *src, uint32_t src_len) {

    buf_write_u32(buf, buf_size, buf_cap, src_len);
    memcpy(buf + *buf_size, src, roundup_4(src_len));
    *buf_size += roundup_4(src_len);
}

static uint32_t buf_read_u32(char **buf, uint64_t *buf_size) {
    uint32_t res = *(uint32_t *)(*buf);
    *buf += sizeof(res);
    *buf_size -= sizeof(res);

    return res;
}

static uint16_t buf_read_u16(char **buf, uint64_t *buf_size) {
    uint16_t res = *(uint16_t *)(*buf);
    *buf += sizeof(res);
    *buf_size -= sizeof(res);

    return res;
}

static void buf_read_n(char **buf, uint64_t *buf_size, char *dst, uint64_t n) {
    memcpy(dst, *buf, n);

    *buf += n;
    *buf_size -= n;
}

static uint32_t wayland_wl_display_get_registry(int fd) {
    uint64_t msg_size = 0;
    char msg[128] = "";
    buf_write_u32(msg, &msg_size, sizeof(msg), wayland_display_object_id);

    buf_write_u16(msg, &msg_size, sizeof(msg),
                  wayland_wl_display_get_registry_opcode);

    uint16_t msg_announced_size =
    wayland_header_size + sizeof(wayland_current_id);
    buf_write_u16(msg, &msg_size, sizeof(msg), msg_announced_size);

    wayland_current_id++;
    buf_write_u32(msg, &msg_size, sizeof(msg), wayland_current_id);

    if ((int64_t)msg_size != send(fd, msg, msg_size, MSG_DONTWAIT))
        exit(1);

    printf("-> wl_display@%u.get_registry: wl_registry=%u\n",
           wayland_display_object_id, wayland_current_id);

    return wayland_current_id;
}

struct state_t create_wayland_window()
{
    printf("connecting to: /run/user/1000/wayland-0\n");

    int server_socket;
    struct sockaddr_un server_addr;

    server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_socket == -1)
        exit(1);

    server_addr.sun_family = AF_UNIX;
    strcpy(server_addr.sun_path, "/run/user/1000/wayland-0");

    if (connect(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr)) == -1)
        exit(1);

    state_t state = {
        .wl_registry = wayland_wl_display_get_registry(server_socket),
        .w = 117,
        .h = 150,
        .stride = 117 * color_channels,
    };

    return state;
}

/*int main() {
    create_wayland_window();
    return 0;
}*/
