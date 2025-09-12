#!/bin/bash
set -e

# This script handles the generation and trust of the ASP.NET Core
# development HTTPS certificate.

# Create the directory if it doesn't exist
mkdir -p /root/.aspnet/https/

echo "Checking for existing ASP.NET Core development certificate..."
if [ ! -f /root/.aspnet/https/aspnetapp.pfx ]; then
  echo "Generating ASP.NET Core development certificate..."
  dotnet dev-certs https -ep /root/.aspnet/https/aspnetapp.pfx -p password
fi

echo "Certificate generation complete. Starting the application..."

# Start the application with hot-reloading. This is the new change.
exec dotnet watch run --urls "http://+:5297"