# README

## Dev environment setup:

Run
`watchexec -r -w ./src/ --workdir ./rs/src cargo run`
and
`./tailwindcss -i ./src/input.css -o ./dist/output.css --watch`
from the top-level directory to enable auto-compilaton of css and html when source files change.
