#!/usr/bin/env bash

scp depl/docker-compose.yml trichter-vps:&
scp depl/.env trichter-vps:&
scp -r depl/config/ trichter-vps:&

wait
