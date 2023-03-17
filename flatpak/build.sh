#!/usr/bin/env bash

set -e

BUILD_DIR=build_dir

rm -f ./galactica

docker build -t galactica:latest .

CID=$(docker create galactica:latest)
docker cp ${CID}:/build/galactica .
docker rm ${CID}

# Use older flatpack gnome platform to get older libssl (newer ones use libssl3, we need libssl1.1)
flatpak install org.gnome.Sdk//3.38
flatpak install org.gnome.Platform//3.38

flatpak-builder --force-clean ${BUILD_DIR} org.s7s.labs.galactica.yml
flatpak-builder --user --install --force-clean ${BUILD_DIR} org.s7s.labs.galactica.yml

rm -rf ${BUILD_DIR}

flatpak run --share=network --filesystem=host org.s7s.labs.galactica login
flatpak run --share=network --filesystem=host org.s7s.labs.galactica code 'What is flatpak?'