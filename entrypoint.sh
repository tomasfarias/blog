#!/bin/sh
# entrypoint.sh

echo "Setting up database"
diesel setup

echo "Running application"
./web
