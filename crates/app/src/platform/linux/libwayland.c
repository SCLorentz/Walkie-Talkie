#include <wayland-client.h>
#include <stdio.h>
#include <string.h>
#include "xdg-shell-client-protocol.h"

struct wl_compositor *compositor;
struct wl_shm *shm;
struct wl_shell *shell;

struct state {
    struct wl_compositor *compositor;
    struct xdg_wm_base *wm_base;
    struct wl_registry * registry;
};

void registry_global_handler
(
    void *data,
    struct wl_registry *registry,
    uint32_t name,
    const char *interface,
    uint32_t version
) {
    struct state *state = data;

    if (strcmp(interface, "wl_compositor") == 0)
        state->compositor = wl_registry_bind(registry, name,
                                &wl_compositor_interface, 4);
    else if (strcmp(interface, "wl_shm") == 0)
        shm = wl_registry_bind(registry, name,
                                &wl_shm_interface, 1);
    else if (strcmp(interface, "xdg_wm_base") == 0)
        state->wm_base =
            wl_registry_bind(registry, name,
                             &xdg_wm_base_interface, 1);
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

static void xdg_surface_configure(
    void *data,
    struct xdg_surface *surface,
    uint32_t serial
) {
    xdg_surface_ack_configure(surface, serial);
}

static const struct xdg_surface_listener xdg_surface_listener = {
    .configure = xdg_surface_configure
};

struct WindowSurface {
    struct wl_display * display;
    struct wl_registry_listener * listener;
    struct wl_surface * surface;
    struct xdg_toplevel * toplevel;
};

struct WindowSurface request_wl_surface(int width, int height)
{
    struct state state = {0};

    struct WindowSurface wl_response = {0};
    wl_response.display = wl_display_connect(NULL);

    if (!wl_response.display)
        return wl_response;

    state.registry = wl_display_get_registry(wl_response.display);
    wl_response.listener = &registry_listener;
    wl_registry_add_listener(state.registry, &registry_listener, &state);

    wl_display_roundtrip(wl_response.display);
    //printf("compositor=%p\n", state.compositor);
    //printf("wm_base=%p\n", state.wm_base);

    wl_response.surface =
        wl_compositor_create_surface(state.compositor);

    struct xdg_surface *xdg_surface =
        xdg_wm_base_get_xdg_surface(state.wm_base, wl_response.surface);

    xdg_surface_add_listener(xdg_surface, &xdg_surface_listener, NULL);

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
