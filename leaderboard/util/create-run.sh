#!/usr/bin/env bash

rate=16
duration=2.875
volume=0.8

auth="Basic dHJpY2h0ZXI6c3VwZXItc2FmZS1wYXNzd29yZA=="

xh POST localhost:5173/api/v1/runs \
    Authorization:"$auth" \
    Content-Type:application/json \
    rate:=$rate \
    duration:=$duration \
    volume:=$volume


