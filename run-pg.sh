#!/usr/bin/env bash

# clear old data
pg_ctl -D ./data stop
rm -f postgres.log

# run new instance
pg_ctl -D ./data -l postgres.log start || exit $?
tail -Fn 20 postgres.log
