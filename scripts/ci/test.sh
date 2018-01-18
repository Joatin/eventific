#!/usr/bin/env bash

echo -e "\033[0;32mRunning tests\033[0m"
./node_modules/.bin/jest
echo -e "\033[0;32mFinished running tests\033[0m"
echo -e "\033[0;32mUploading coverage result\033[0m"
./node_modules/.bin/codecov
echo -e "\033[0;32mFinished uploading coverage\033[0m"