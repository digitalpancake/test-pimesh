#!/bin/bash

## packages list
# jq
# curl
# wget

echo "Starting Script!"

mkdir /tmp/pimesh
cd /tmp/pimesh

## Connect to server
## Store ssh in /root/.ssh
## SSID and Password to server stored in /etc/pimesh/internet

nmcli radio wifi on
nmcli d wifi connect $SSID password $NW_PASS

IP = 192.168.1.1
PORT = 9999

curl http://$IP:$PORT/api/register | jq -re

git clone https://github.com/digitalpancake/test-pimesh.git
sudo apt install git openssh make networkmanager

# Run damon alburn
