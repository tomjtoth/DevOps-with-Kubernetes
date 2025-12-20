#!/bin/sh

UUID=$(uuidgen)

while :
do
	echo "$(date): $UUID"
	sleep 5
done
