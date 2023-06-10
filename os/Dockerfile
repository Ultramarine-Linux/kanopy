FROM ghcr.io/ultramarine-linux/base-ostree:38

# OSTree image

# Install packages

RUN rpm-ostree install --idempotent --allow-inactive \
    git \
    nano \
    containerd \
    runc \
    kubernetes-client \
    kubernetes-kubeadm

COPY ./assets/ /var/lib/kanopy

RUN ostree container commit