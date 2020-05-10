#!/bin/bash

# imports utils
source scripts/utils/cecho.bash; # imports functions: cecho, info, warning, error 

# yes/no confirmation input
inquire ()  {
  local reply="";
  local trials=1;
  while [[ -z $reply ]]; do
    if [[ $trials -eq 1 ]]; then
      echo -n "$1  [y/n]: " >&2;
    else
      error -n "$1  [y/n]: " >&2;
    fi
    read reply;
    case $reply in
      y | Y | yes | YES | Yes ) 
        reply="y";
      ;;
      n | N | no | NO | N ) 
        reply="n";
      ;;
      *) 
        reply="";
        ((trials++));
      ;;
    esac
  done
  echo $reply;
}

# check for cargo
which cargo &> /dev/null
if [[ "$?" == 1 ]]; then
  error "❗ Cargo not found. Consider checking your Rust installation."
  exit 1;
fi

# check for watchexec
which watchexec &> /dev/null
if [[ "$?" != 0 ]]; then
  error -n "❗ watchexec not found. ";
  input=$(inquire "Install now?");
  if [[ $input == 'y' ]]; then
    # install watchexec
    if [[ "$OSTYPE" == "darwin"* ]]; then
      # Mac OSX
      brew install watchexec;
    else
      info "Installing watchexec...";
      cargo install watchexec;
    fi
    watchexec -r -s SIGKILL --exts rs,toml cargo run;
  else
    cargo run;
    warning "❗ watchexec not installed. You will have to rebuild project manually.";
  fi
else
  watchexec -r -s SIGKILL --exts rs cargo run;
fi