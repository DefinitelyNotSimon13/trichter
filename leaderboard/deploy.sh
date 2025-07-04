#!/usr/bin/env bash

az containerapp compose create \
    --environment trichter-app-env \
    --resource-group trichter-rg \
    --compose-file-path "depl/docker-compose.yaml"
