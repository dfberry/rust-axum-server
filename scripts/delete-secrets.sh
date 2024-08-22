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

# Delete each secret
for secret in $secrets; do

  echo "Removing secret $secret"

  if [[ $secret == APP_* ]]; then

    echo "Removing secret $secret"

    az containerapp secret remove \
      --subscription $AZ_SUB_ID \
      --name $AZ_APP_NAME \
      --resource-group $AZ_RG \
      --secret-names $secret
  fi
done