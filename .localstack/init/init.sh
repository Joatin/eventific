#!/usr/bin/env bash
awslocal sns create-topic --name eventific
awslocal sqs create-queue --queue-name eventific
