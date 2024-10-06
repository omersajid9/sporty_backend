#!/usr/bin/env bash

# run postgres server
docker-compose up -d;

# wait for postgres server to start
sleep 1;

# run migrations
sqlx migrate run;

cd api;

# compile server
cargo check;

#run server
cargo watch -c -x run;
