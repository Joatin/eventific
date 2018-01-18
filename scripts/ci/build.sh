#!/usr/bin/env bash

echo -e "\033[0;32mBuilding\033[0m"
./node_modules/.bin/lerna run build
echo -e "\033[0;32mFinished building\033[0m"