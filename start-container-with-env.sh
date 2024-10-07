# load_container_and_run.sh
#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    printf "Loading environment variables from .env file\n"

    # Run your application, export port 4000
    docker run -p 4000:4000 --env-file .env source-board-server
else
    printf "No .env file found\n"
    exit 1
fi

