#!/bin/sh -ex

cargo build

where_am_i=$(cd `dirname $0`/..; pwd)
PATH="$where_am_i/target/debug:$PATH"

protoc_ver=$(protoc --version)
case "$protoc_ver" in
    "libprotoc 3"*)
        HAS_PROTO3=true
        ;;
    *)
        echo "skipping tests for protobuf 3, because protoc version is ${protoc_ver}"
        HAS_PROTO3=false
        ;;
esac

rm -f test/*/pb_*.rs

protoc --rust_out test/v2 -I test/v2 test/v2/*.proto
if $HAS_PROTO3; then
    protoc --rust_out test/v3 -I test/v3 test/v3/*.proto
else
    # Because `#[cfg(nonexistent)]` still requires module files to exist
    for f in test/v3/*.proto; do
        f=${f%.proto}
        cat < /dev/null > $f.rs
    done
fi

(
    cd test

    if $HAS_PROTO3; then
        rustc --cfg proto3 --test -L ../../target/debug lib.rs
    else
        rustc --test -L ../../target/debug lib.rs
    fi
    ./lib
)
