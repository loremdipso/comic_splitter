#!/bin/bash

filewatcher ./src/ 'rm ./test/output/* ; printf "\ec" && ./test.sh'

