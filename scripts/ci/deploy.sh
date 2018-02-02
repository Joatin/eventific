#!/usr/bin/env bash
set -e

echo "//registry.npmjs.org/:_authToken=\${NPM_TOKEN}" > ~/.npmrc
if [ $TRAVIS_BRANCH == 'develop' ]; then
  if [ $TRAVIS_PULL_REQUEST == "false" ]; then
    echo -e "\033[0;32mDeploying beta build\033[0m"
    ./node_modules/.bin/lerna publish --canary=beta --npm-tag=beta --yes --npm-client npm
    echo -e "\033[0;32mFinished deploying\033[0m"
  fi
elif [ $TRAVIS_BRANCH == "master" ]; then
  if [ $TRAVIS_PULL_REQUEST == "false" ]; then
    echo -e "\033[0;32mDeploying\033[0m"
    git config --global user.email "travis@travis-ci.org"
    git config --global user.name "Travis CI"
    git remote set-url origin https://${GH_TOKEN}@github.com/eventific/eventific.git > /dev/null 2>&1
    git checkout -B master
    ./node_modules/.bin/lerna publish --conventional-commits -m "chore(release): [ci skip] publish %s" --yes --npm-client npm
    git add --all

    echo -e "\033[0;32mFinished deploying\033[0m"
  fi
else
  if [ $TRAVIS_PULL_REQUEST == "false" ]; then
    echo -e "\033[0;32mDeploying alpha build\033[0m"
    ./node_modules/.bin/lerna publish --canary=beta --npm-tag=alpha --yes --npm-client npm
    echo -e "\033[0;32mFinished deploying\033[0m"
  fi
fi
