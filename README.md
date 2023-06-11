# Kanopy

Ultramarine Kanopy is a lightweight and easy to setup operating system optimized for Kubernetes.

It is based on Ultramarine Linux, a Fedora remix with a focus on ease of use hardware support with proprietary drivers and firmware included.

Kanopy is based on the [Fedora CoreOS](https://coreos.fedoraproject.org/) project and is designed to be a plug and play operating system for managing Kubernetes clusters.

You can run Kanopy on a virtual machine, on a bare metal server, or even a Raspberry Pi. It automatically configures itself to start a Kubernetes cluster with a configuration file, and initializes the cluster on first boot.

This repository contains the Kanopy helper service, which is responsible for bootstrapping and managing the cluster, and the scripts to build a Kanopy image.

Kanopy uses the vanilla Kubernetes distribution, and has options to install various CNIs and other addons such as:

- [Calico](https://www.projectcalico.org/)
- [Cilium](https://cilium.io/)
- [Flannel](https://github.com/flannel-io/flannel)
- [Traefik](https://traefik.io/)
- [Rancher](https://rancher.com/)

...and more!

## todo
- [x] Implement config schema
- [x] Apply configs to cluster (WIP)
- [x] Finish writing helper (WIP)
- [ ] Add YAML include macros
- [ ] Build web UI for cluster management
- [ ] Add support for more addons
- [ ] Helm chart manager in web UI

## Getting Started

At the moment, Kanopy is still in development and does not yet have a bootable image release, only the OSTree build is available through GHCR.

To build a CoreOS image out of the Docker image, install [coreos-assembler](https://coreos.github.io/coreos-assembler/)

> NOTE: Currently, building CoreOS images are not yet supported, see https://github.com/coreos/fedora-coreos-tracker/issues/1151. In the meanwhile, you can rebase from an existing CoreOS installation. And we will work on a COSA project to build Kanopy images.

```bash
rpm-ostree rebase --reboot ostree-unverified-registry:ghcr.io/ultramarine/kanopy:38
```

Make sure to install the configuration file to `/etc/kanopy/config.yaml` before rebasing.

You can also write an Ignition config to write the config first, then rebase to Kanopy, and then reboot.
