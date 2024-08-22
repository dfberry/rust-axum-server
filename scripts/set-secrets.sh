#!/bin/bash

# SECRETS can only be 20 chars

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

# Function to transform secret names to valid Azure Container App secret names
transform_secret_name() {
  local name=$1
  # Replace invalid characters with underscores, convert to lowercase, and replace underscores with dashes
  name=$(echo $name | sed 's/[^a-zA-Z0-9_]/_/g' | tr '[:upper:]' '[:lower:]' | tr '_' '-')
  echo $name
}

# Read the .env.local file and set each value as a secret
while IFS= read -r line || [ -n "$line" ]; do

  # Split the line into key and value
  key=$(echo "$line" | cut -d '=' -f 1)
  value=$(echo "$line" | cut -d '=' -f 2-)

  echo "Setting secret $key"

  # Remove quotes from the value if present
  value=$(echo $value | sed 's/^"//;s/"$//')
  
  # Transform the secret name to a valid one
  TRANSFORMED_KEY=$(transform_secret_name $key)
  
  # Set the secret in Azure Container App
  az containerapp secret set \
    --subscription $AZ_SUB_ID \
    --name $AZ_APP_NAME \
    --resource-group $AZ_RG \
    --secrets $TRANSFORMED_KEY=$value

  # Set the environment variable in Azure Container App
  az containerapp update \
    --subscription $AZ_SUB_ID \
    --name $AZ_APP_NAME \
    --resource-group $AZ_RG \
    --set-env-vars $key=secretref:$TRANSFORMED_KEY
  
done < ../.env