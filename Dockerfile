FROM rust:latest as builder

COPY . /kanopy

WORKDIR /kanopy

RUN cargo build --release


FROM ghcr.io/ultramarine-linux/base-ostree:38 as runtime

# copy files
COPY os/override /
COPY os/assets/ /var/lib/kanopy

# Install packages

RUN rpm-ostree install --idempotent --allow-inactive \
    git \
    nano \
    cri-o \
    runc \
    kubernetes-client \
    kubernetes-kubeadm \
    helm

COPY --from=builder /kanopy/target/release/kanopy /usr/bin/kanopy
