#!/bin/bash

## Prerequisites
## az login --use-device-code
##

# Set variables - only use alphanumeric characters (no dashes or underscores)
AZURE_SUBSCRIPTION_ID="19016922-4bf5-4c41-9553-8eff5da1500e"
AZURE_RESOURCE_GROUP="rust-axum-server"
AZURE_CONTAINER_APP_NAME="dfberrycontainerapp"

az containerapp show \
--subscription $AZURE_SUBSCRIPTION_ID \
--name $AZURE_CONTAINER_APP_NAME \
--resource-group $AZURE_RESOURCE_GROUP \
--query properties.configuration.ingress.fqdn \
--output tsv