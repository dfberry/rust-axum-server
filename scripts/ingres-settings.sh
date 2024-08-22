#!/bin/bash

## Prerequisites
## az login --use-device-code
##

DOTENV_PATH="../.env"

# Load environment variables from .env file into the script's environment
if [ -f $DOTENV_PATH ]; then
  set -a
  source $DOTENV_PATH
  set +a
else
  echo "Error: .env file not found at $DOTENV_PATH"
  exit 1
fi

# Debug: Display all environment variables loaded from .env
echo "Loaded environment variables from $DOTENV_PATH:"
while IFS='=' read -r key value; do
  echo "$key=$value"
done < $DOTENV_PATH


az containerapp ingress show \
--subscription $AZ_SUB_ID \
--name $AZ_APP_NAME \
--resource-group $AZ_RG 
