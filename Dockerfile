FROM rust:latest as builder

COPY . /kanopy

WORKDIR /kanopy

RUN cargo build --release


FROM ghcr.io/ultramarine-linux/base-ostree:38 as runtime

# copy files
COPY os/override /
COPY os/assets/ /var/lib/kanopy

# Install packages
# if not x86_64, remove akmods-secureboot
RUN if [ "$(uname -m)" != "x86_64" ]; then \
    rm -f /etc/yum.repos.d/akmods-secureboot.repo; \
    fi

RUN rpm-ostree install --idempotent --allow-inactive \
    git \
    nano \
    cri-o \
    runc \
    crun \
    youki \
    kubernetes-client \
    kubernetes-kubeadm \
    helm

COPY --from=builder /kanopy/target/release/kanopy /usr/bin/kanopy
