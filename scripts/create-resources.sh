#!/bin/bash

## Prerequisites
## az login --use-device-code
##

# Set variables
AZURE_SUBSCRIPTION_ID="your-subscription-id"
AZURE_RESOURCE_GROUP="your-resource-group"
AZURE_LOCATION="your-location"

AZURE_CONTAINER_REGISTRY_NAME="your-acr-name"
AZURE_CONTAINER_APP_NAME="your-container-app-name"

# Create Azure resource group
az group create --subscription $AZURE_SUBSCRIPTION_ID --name $AZURE_RESOURCE_GROUP --location $AZURE_LOCATION

# Create Azure container registry
az acr create --subscription $AZURE_SUBSCRIPTION_ID --resource-group $AZURE_RESOURCE_GROUP --name $AZURE_CONTAINER_REGISTRY_NAME --sku Basic

# Create Azure container app
az containerapp create --subscription $AZURE_SUBSCRIPTION_ID --resource-group $AZURE_RESOURCE_GROUP --name $AZURE_CONTAINER_APP_NAME --environment ${AZURE_CONTAINER_APP_NAME}-env