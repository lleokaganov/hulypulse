#!/bin/bash

clear

#git push origin --delete feature/insert_update
#git push origin --delete feature/insert_update_uuid

# cargo fmt

#NAME="final_tears"

# git checkout -b feature/${NAME}
git add .
#git commit -m "First test redis 5 sec, use: http://127.0.0.1:8095/api/"
git commit -m "HTTP full release: get, put, delete, list(get?prefix=...)"
git push origin
# feature/${NAME}
