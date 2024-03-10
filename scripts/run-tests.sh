#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

touch_env=0

while (($#)); do
    [[ $1 = -- ]] && {
        shift
        break
    }
    [[ $1 = -?* ]] || break
    case $1 in
    --touch-env) touch_env=1 ;;
    -*) invalid_option "$1" ;;
    esac
    shift
done

if [[ $touch_env -eq 1 ]]; then
    touch .env.local
fi

docker compose up postgres --wait -d

diesel migration run

cargo test

docker compose down
