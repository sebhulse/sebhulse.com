# README

## Dev environment setup:

On Mac, Run
`watchexec -r -w ./src/ bash tools/build.sh`
and
`tools/tailwindcss-mac-x64 -i ./src/input.css -o ./dist/output.css --watch `
from the top-level directory to enable auto-compilaton of css and html when source files change.
