# Kanopy CoreOS Deployment Guide

This guide contains instructions, example configs and scripts to prepare a CoreOS image and/or an ignition config to deploy Kanopy.

As CoreOS Assembler does not support building from preexisting OCI images, we will be using an existing CoreOS image and automatically rebasing it to Kanopy.

## Getting Started

### Prerequisites

- Podman
- Basic knowledge of CoreOS and Ignition


### Installing

Clone this repository and install the dependencies:

```bash
git clone https://github.com/ultramarine-linux/kanopy.git
cd kanopy/deploy
```

