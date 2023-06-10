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
- [] Implement config schema
- [] Apply configs to cluster
- [] Finish writing helper
- [] Add YAML include macros


## Getting Started

TO BE WRITTEN