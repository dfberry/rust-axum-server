#!/bin/bash

DOTENV_PATH="../.env"

# Check if the first parameter is empty
if [ -z "$1" ]; then
  echo "First parameter is empty, assuming environment is already set."
else
  # Load environment variables from .env file into the script's environment
  if [ -f $DOTENV_PATH ]; then
    set -a
    source $DOTENV_PATH
    set +a
  else
    echo "Error: .env file not found at $DOTENV_PATH"
    exit 1
  fi
fi


# Variables
RESOURCE_GROUP=$AZ_RG
CONTAINER_APP_NAME=$AZ_APP_NAME

# Get the latest revision name
LATEST_REVISION=$(az containerapp revision list \
  --name $CONTAINER_APP_NAME \
  --resource-group $RESOURCE_GROUP \
  --query "[0].name" \
  -o tsv)

if [ -z "$LATEST_REVISION" ]; then
  echo "Error: No revisions found for the container app."
  exit 1
fi

# Get all information about the latest revision and display it
REVISION_INFO=$(az containerapp revision show \
  --name $CONTAINER_APP_NAME \
  --resource-group $RESOURCE_GROUP \
  --revision $LATEST_REVISION \
  -o json)

#echo "Latest revision information: $REVISION_INFO"

# Extract properties from the revision information
RESOURCE_GROUP=$(echo $REVISION_INFO | jq -r '.resourceGroup')
ACTIVE=$(echo $REVISION_INFO | jq -r '.properties.active')
CREATED_TIME=$(echo $REVISION_INFO | jq -r '.properties.createdTime')
HEALTH_STATE=$(echo $REVISION_INFO | jq -r '.properties.healthState')
PROVISIONING_STATE=$(echo $REVISION_INFO | jq -r '.properties.provisioningState')
RUNNING_STATE=$(echo $REVISION_INFO | jq -r '.properties.runningState')

echo "Latest revision: $LATEST_REVISION"
echo "Active: $ACTIVE"
echo "Created Time: $CREATED_TIME"
echo "Health State: $HEALTH_STATE"
echo "Provisioning State: $PROVISIONING_STATE"
echo "Running State: $RUNNING_STATE"
echo "Resource Group: $RESOURCE_GROUP"

if [ "$PROVISIONING_STATE" != "Provisioned" ]; then
  echo "Error: The latest revision did not succeed."
  exit 1
else
  echo "The latest revision is Provisioned."
fi