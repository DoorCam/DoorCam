#!/bin/zsh

rm db_template.sqlite
touch db_template.sqlite

echo ".open db_template.sqlite\n.read scheme.sql\n.quit" | sqlite3
