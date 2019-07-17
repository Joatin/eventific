#!/usr/bin/env bash
awslocal sns create-topic --name eventific
awslocal sqs create-queue --queue-name eventific
awslocal dynamodb create-table --attribute-definitions "[{\"AttributeName\": \"aggregateId\", \"AttributeType\": \"S\"},{\"AttributeName\": \"eventId\", \"AttributeType\": \"N\"}]" --table-name eventific --key-schema "[{\"AttributeName\": \"aggregateId\", \"KeyType\": \"HASH\"},{\"AttributeName\": \"eventId\", \"KeyType\": \"RANGE\"}]" --provisioned-throughput "ReadCapacityUnits=5,WriteCapacityUnits=5"
