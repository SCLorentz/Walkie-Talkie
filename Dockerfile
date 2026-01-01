# Build env for linux to use with Podman
# TODO: fix the "libssl not found" (it's installed, idk why it's not finding)
FROM debian:stable-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
	clang \
	build-essential \
	lld \
	libvulkan-dev \
	vulkan-tools \
	vulkan-validationlayers \
	ca-certificates \
	pkg-config \
	rustup \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

RUN rustup default stable && rustup update

WORKDIR /workspace

COPY . /workspace

CMD ["/bin/bash"]
