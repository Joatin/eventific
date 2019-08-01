#!/usr/bin/env bash
sleep 3
awslocal sns create-topic --name eventific
awslocal sqs create-queue --queue-name eventific
