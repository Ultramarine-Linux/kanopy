FROM ghcr.io/ultramarine-linux/base-ostree:38

# OSTree image

# Install packages


RUN rpm-ostree install --idempotent --allow-inactive \
    git \
    nano \
    cri-o \
    runc \
    kubernetes-client \
    kubernetes-kubeadm

COPY ./override /
COPY ./assets/ /var/lib/kanopy
