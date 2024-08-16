#!/bin/bash

## Prerequisites
## az login --use-device-code
##

# Set variables - only use alphanumeric characters (no dashes or underscores)
AZURE_SUBSCRIPTION_ID="19016922-4bf5-4c41-9553-8eff5da1500e"

az ad sp create-for-rbac \
--name "CICD-rust-2" \
--role contributor \
--scopes /subscriptions/$AZURE_SUBSCRIPTION_ID \
--sdk-auth