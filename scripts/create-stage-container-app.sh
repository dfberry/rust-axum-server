#!/bin/bash

## Prerequisites
## az login --use-device-code
##

# Set variables - only use alphanumeric characters (no dashes or underscores)
AZURE_SUBSCRIPTION_ID="19016922-4bf5-4c41-9553-8eff5da1500e"
AZURE_RESOURCE_GROUP="rust-axum-server"
AZURE_LOCATION="eastus2"

AZURE_CONTAINER_REGISTRY_NAME="dfberryregistry"
AZURE_CONTAINER_APP_ENV_NAME="managedEnvironment-rustaxumserver-a56d"
AZURE_CONTAINER_APP_NAME="dfberrycontainerapp-stage"
TARGET_PORT=4000

# Create Azure resource group
#az group create --subscription $AZURE_SUBSCRIPTION_ID --name $AZURE_RESOURCE_GROUP --location $AZURE_LOCATION

# Create Azure container registry
#az acr create --subscription $AZURE_SUBSCRIPTION_ID --resource-group $AZURE_RESOURCE_GROUP --name $AZURE_CONTAINER_REGISTRY_NAME --sku Basic

# Create Azure Container App environment
#az containerapp env create --subscription $AZURE_SUBSCRIPTION_ID --resource-group $AZURE_RESOURCE_GROUP --name $AZURE_CONTAINER_APP_ENV_NAME --location $AZURE_LOCATION

# Create Azure container app
az containerapp create --subscription $AZURE_SUBSCRIPTION_ID --resource-group $AZURE_RESOURCE_GROUP --name $AZURE_CONTAINER_APP_NAME --environment ${AZURE_CONTAINER_APP_ENV_NAME}

# Enable ingress for Azure Container App
az containerapp ingress enable \
--subscription $AZURE_SUBSCRIPTION_ID \
--name $AZURE_CONTAINER_APP_NAME \
--resource-group $AZURE_RESOURCE_GROUP \
--target-port $TARGET_PORT \
--transport http \
--type external 