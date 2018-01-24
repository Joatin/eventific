#!/usr/bin/env bash
set -e

echo "//registry.npmjs.org/:_authToken=\${NPM_TOKEN}" > ~/.npmrc
if [ $TRAVIS_BRANCH == 'develop' ]; then
    echo -e "\033[0;32mDeploying canary build\033[0m"
    ./node_modules/.bin/lerna publish --canary --yes --npm-client npm
    echo -e "\033[0;32mFinished deploying\033[0m"
fi
if [ $TRAVIS_BRANCH == 'master' ]; then
    echo -e "\033[0;32mDeploying\033[0m"
    ./node_modules/.bin/lerna publish --conventional-commits --yes--npm-client npm
    echo -e "\033[0;32mFinished deploying\033[0m"
fi
