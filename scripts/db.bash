#!/bin/bash

# imports utils
source ./scripts/utils/cecho.bash # imports functions: cecho, info, warning, error 

# parses environment variables
source .env
# creates postgres docker container if not already
DOCKER_CONTAINER=quantiz_postgres
DOCKER_IMAGE=postgres
EXISTING_CONTAINER_ID=`docker ps -aq --filter name=$DOCKER_CONTAINER`

if ! test -z "$EXISTING_CONTAINER_ID"; then
  info "\"$DOCKER_CONTAINER\" docker container already exists..."
else
  info "creating \"$DOCKER_CONTAINER\" docker container..."
  docker run --rm --name $DOCKER_CONTAINER -d --env-file .env -p $POSTGRES_PORT:$POSTGRES_PORT $DOCKER_IMAGE > /dev/null
  docker ps | grep -E "IMAGE|$DOCKER_CONTAINER" --color=always
fi

# connects to postgres and setup diesel
# eval $(egrep -v '^#' .env | xargs)
if diesel setup --database-url $DATABASE_URL; then
  info "connected diesel to $DATABASE_URL"
fi
  