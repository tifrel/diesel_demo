#!/usr/bin/env bash

# clear old data
pg_ctl -D ./data stop
rm -rf data postgres.log migrations diesel.toml
mkdir data

# run new instance
initdb ./data || exit $?
pg_ctl -D ./data -l postgres.log start || exit $?

diesel setup || exit $?
diesel migration generate init_db || exit $?
init_db_dir=$(echo migrations/*_init_db)
cp init_db_up.sql "$init_db_dir/up.sql"
cp init_db_down.sql "$init_db_dir/down.sql"
diesel migration run || exit $?

tail -Fn 20 postgres.log
