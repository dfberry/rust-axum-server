#!/bin/bash

# filepath: /workspaces/rust-axum-server/start-dev-with-env.sh
# Load environment variables from .env file, print them, then run the application

ENV_FILE=".env"

if [ -f "$ENV_FILE" ]; then
    # Read the contents of the .env file into a variable
    DOTENV_CONTENT=$(cat "$ENV_FILE")
    
    # Print out the value of the variable
    echo "Contents of $ENV_FILE file:"
    echo "$DOTENV_CONTENT"
    
    # Export environment variables (ignoring comment lines)
    export $(echo "$DOTENV_CONTENT" | grep -v '^#' | xargs)
    
    # Print all environment variables for debugging
    echo "===== Runtime environment variables ====="
    env | sort
fi

# Run your application
cargo build && cargo run