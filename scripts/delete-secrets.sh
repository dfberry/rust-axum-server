#!/bin/bash

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

# Get the list of environment variables
az containerapp update \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --remove-all-env-vars

# Get the list of secrets
secrets=$(az containerapp secret list \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --query "[].name" -o tsv)

echo "Secrets: $secrets"

# Variable to hold concatenated secrets for removal
secrets_to_remove=""

# Build up the list of secrets to remove
for secret in $secrets; do
  echo "Processing secret $secret"
  secrets_to_remove="$secrets_to_remove $secret"
done

# Remove the secrets in a single command
az containerapp secret remove \
  --subscription $AZ_SUB_ID \
  --name $AZ_APP_NAME \
  --resource-group $AZ_RG \
  --secret-names $secrets_to_remove