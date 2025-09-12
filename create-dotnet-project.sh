# bash script to create a .NET 9.0 Web API project using Docker
# Give execute permission:      chmod +x create-dotnet-project.sh
# Run the script:               ./create-dotnet-project.sh

#!/bin/bash
PROJECT_DIR="$(dirname "$(realpath "$0")")/dotnet-api"
#PROJECT_DIR="dotnet-api"
echo "Creating the .NET 9.0 Web API project in the '$PROJECT_DIR' directory..."
if [ -d "$PROJECT_DIR" ]; then
echo "Existing '$PROJECT_DIR' directory found. Deleting it..."
rm -rf "$PROJECT_DIR"
fi
mkdir "$PROJECT_DIR"
docker run --rm -v "$(pwd -W)/$PROJECT_DIR":/app mcr.microsoft.com/dotnet/sdk:9.0 dotnet new webapi --output /appecho "Project files have been created in the '$PROJECT_DIR' directory."
echo "You can now run 'docker compose up' to build and run your services."

