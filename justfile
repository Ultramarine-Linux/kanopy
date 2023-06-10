# idk insert some build scripts here

image := "ghcr.io/ultramarine-linux/kanopy"


build:
    docker build -t {{image}} .

push: build
    docker push {{image}}