# load_env_and_run.sh
#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

# Run your application
docker run --env-file .env source-board-server