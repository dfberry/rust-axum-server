#!/bin/bash

## Prerequisites
## az login --use-device-code
##

# Set variables - onlu use alphanumeric characters (no dashes or underscores)
AZURE_SUBSCRIPTION_ID="<your-subscription-id>"

az ad sp create-for-rbac \
--name "CICD" \
--role contributor \
--scopes /subscriptions/$AZURE_SUBSCRIPTION_ID \
--sdk-auth