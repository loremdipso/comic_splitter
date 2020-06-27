#!/bin/bash

filewatcher ./src/ 'rm ./test/output/* ; printf "\ec" && time cargo run --release -q ./test/input/test.png "./test/input/test (copy).png" --output ./test/output'
#filewatcher ./src/ 'rm ./test/output/* ; printf "\ec" && time cargo run --release -q ./test/input/*.png --output ./test/output'

#filewatcher ./src/ 'rm ./test/output/* ; printf "\ec" && cargo run --release -q ./test/input/test_alt.png --output ./test/output --delete'

#filewatcher ./src/ 'rm ./test/output/* ; printf "\ec" && cargo run --release -q ./test/input/test.png --output ./test/output'

#filewatcher ./src/ 'rm ../test/output/* ; printf "\ec" && cargo run -q ../test/input/test.png --output ../test/output && echo $(date)'
#filewatcher ./src/ 'printf "\ec" && cargo run -q ../test/input/*png --output /home/mikhail/Projects/comic_splitter/src/output'
