#!/usr/bin/env bash
set -e

echo -e "\033[0;32mInstalling dependencies\033[0m"
yarn install --frozen-lockfile --non-interactive
./node_modules/.bin/lerna bootstrap
echo -e "\033[0;32mFinished installing\033[0m"
