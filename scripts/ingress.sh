#!/bin/bash

## Prerequisites
## az login --use-device-code
##

# Set variables - onlu use alphanumeric characters (no dashes or underscores)

AZURE_SUBSCRIPTION_ID="<your-subscription-id>"
AZURE_RESOURCE_GROUP="<your-resource-group>"
AZURE_CONTAINER_APP_NAME="<your-container-app-name>"
TARGET_PORT=3000

az containerapp ingress enable \
--subscription $AZURE_SUBSCRIPTION_ID \
--name $AZURE_CONTAINER_APP_NAME \
--resource-group $AZURE_RESOURCE_GROUP \
--target-port $TARGET_PORT \
--transport http \
--type external 