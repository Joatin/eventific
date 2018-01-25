#!/usr/bin/env bash
set -e

echo -e "\033[0;32mRunning tslint\033[0m"
./node_modules/.bin/tslint 'packages/**/*.ts' -e '**/__mocks__/**/*.ts' -e '**/*.d.ts' -e '**/*.tests.ts' -e '**/lib/*'
echo -e "\033[0;32mFinished running tslint\033[0m"
echo -e "\033[0;32mRunning tests\033[0m"
./node_modules/.bin/jest --ci --coverage --testPathPattern packages
echo -e "\033[0;32mFinished running tests\033[0m"
echo -e "\033[0;32mUploading coverage result\033[0m"
./node_modules/.bin/codecov
echo -e "\033[0;32mFinished uploading coverage\033[0m"
echo -e "\033[0;32mRunning e2e tests\033[0m"
./node_modules/.bin/jest --ci --testPathPattern integration
echo -e "\033[0;32mFinished running tests\033[0m"
