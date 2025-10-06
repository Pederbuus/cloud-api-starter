# ROOT check that homepage returns something
Invoke-RestMethod -Uri "http://localhost:3000/"

# Vehicle
# Invoke-RestMethod -Uri "http://localhost:3000/vehicle" -Method Get

# Create a new vehicle
# Invoke-RestMethod -Uri "http://localhost:3000/vehicle" -Method Post

# Create a new vehicle using: Json
# Using the wrong fields fails the whole request
$Params = @{
    Uri = 'http://localhost:3000/vehicle'
    Method = 'Post'
    Body = @{
        make = "Tesla"
        model = "Model S"
        year = 2020
    } | ConvertTo-Json
    ContentType = 'application/json'
}
Invoke-RestMethod @Params

# Create a new vehicle using: Query
# Deserializing with the wrong content/fields fails the whole request
$Params = @{
    Uri = 'http://localhost:3000/vehicle/query?make=Tesla&model=Model S&year=2020&name=John&email=john@example.com'
    Method = 'Post'
}
Invoke-RestMethod @Params

# Get vehicles using: Query
$Params = @{
    Uri = 'http://localhost:3000/vehicle?'
    Method = 'Get'
}
Invoke-RestMethod @Params

# Get vehicles using: Query
$Params = @{
    Uri = 'http://localhost:3000/vehicle/f8486382-5a9c-4af1-bd08-ab24ca969cac'
    Method = 'Delete'
}
Invoke-RestMethod @Params

# PUT vehicle - with StatusVariable
# 1. Define the parameters (remove StatusVariable)
$Params = @{
    Uri = 'http://localhost:8080/vehicle/f8486382-5a9c-4af1-bd08-ab24ca969cac'
    Method = 'Put'
    Headers = @{ 'Accept' = 'application/json' }
    Body = @{
        make = "Toyota1"
        model = "Corolla"
        year = 2024
    } | ConvertTo-Json
    ContentType = 'application/json'
    # Remove StatusVariable = 'httpStatus'
}

# 2. Use Invoke-WebRequest and save the entire response object
$Response = Invoke-WebRequest @Params

# 3. Access the status code property
Write-Host "HTTP Status Code: $($Response.StatusCode)"

# 4. Access the content (JSON body)
# NOTE: The Content property is a string, so you must convert it back to an object
$ResponseObject = $Response.Content | ConvertFrom-Json
$ResponseObject


$Params = @{
    Uri = "http://localhost:8080/vehicle"
    Method = 'POST'
    Headers = @{ 'Accept' = 'application/json' }
    Body = @{
        make = "New Make"
        model = "New Model"
        year = 2025
    } | ConvertTo-Json
    ContentType = 'application/json'
}
# Execute the update
Invoke-RestMethod @Params