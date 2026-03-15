#include <wayland-client.h>
#include <stdio.h>
#include <string.h>
#include "xdg-shell-client-protocol.h"

struct wl_compositor *compositor;
struct wl_shm *shm;
struct wl_shell *shell;

void registry_global_handler
(
    void *data,
    struct wl_registry *registry,
    uint32_t name,
    const char *interface,
    uint32_t version
) {
    if (strcmp(interface, "wl_compositor") == 0)
        compositor = wl_registry_bind(registry, name,
                                &wl_compositor_interface, 3);
    else if (strcmp(interface, "wl_shm") == 0)
        shm = wl_registry_bind(registry, name,
                                &wl_shm_interface, 1);
    else if (strcmp(interface, "wl_shell") == 0)
        shell = wl_registry_bind(registry, name,
                                &wl_shell_interface, 1);
}

void registry_global_remove_handler
(
    void *data,
    struct wl_registry *registry,
    uint32_t name
) {
    printf("removed: %u\n", name);
}

struct wl_registry_listener registry_listener = {
    .global = registry_global_handler,
    .global_remove = registry_global_remove_handler
};

struct state {
    struct wl_compositor *compositor;
    struct xdg_wm_base *wm_base;
};

struct WindowSurface {
    struct wl_display * display;
    struct wl_registry * registry;
    struct wl_registry_listener * listener;
    struct wl_surface * surface;
    struct xdg_toplevel * toplevel;
};

struct WindowSurface request_wl_surface(int width, int height)
{
    struct state state = {0};

    struct WindowSurface wl_response;
    wl_response.display = wl_display_connect(NULL);
    wl_response.registry = wl_display_get_registry(wl_response.display);
    wl_response.listener = &registry_listener;
    wl_registry_add_listener(wl_response.registry, &registry_listener, &state);

    wl_display_roundtrip(wl_response.display);

    wl_response.surface =
        wl_compositor_create_surface(compositor);

    struct xdg_surface *xdg_surface =
        xdg_wm_base_get_xdg_surface(state.wm_base, wl_response.surface);

    wl_response.toplevel =
        xdg_surface_get_toplevel(xdg_surface);

    xdg_toplevel_set_title(wl_response.toplevel, "title");
    wl_surface_commit(wl_response.surface);

    return wl_response;
}

void loop_wl_event(struct wl_display *display)
{
    while (1) {
        wl_display_dispatch(display);
    }
}

void request_wl_disconnect(struct wl_display *display)
{
    wl_display_disconnect(display);
}
