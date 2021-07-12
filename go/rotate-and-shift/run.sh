#!/bin/bash

(for prob in {1..132}; do echo "working on ${prob}";go run main.go -v -p ${prob} | grep -q "SOLVED" || rm solutions/${prob}.json;done)