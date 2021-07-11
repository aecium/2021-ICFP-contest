#!/bin/bash

(for prob in {1..132}; do go run main.go ${prob} | grep -q "SOLVED" && echo ${prob}>>../solved || rm solutions/${prob}.json;done)