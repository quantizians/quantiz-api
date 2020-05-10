#!/bin/bash

# imports utils
source scripts/utils/cecho.bash; # imports functions: cecho, info, warning, error 

info "Setting up git hooks";
# copy hooks into .git/hooks
cp -a scripts/.githooks/. .git/hooks/;
# give executable permission
chmod +x -R .git/hooks;