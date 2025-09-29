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

# PUT vehicle - with StatusVariable
# 1. Define the parameters (remove StatusVariable)
$Params = @{
    Uri = 'http://localhost:3000/vehicle/3e68be47-4e1f-4b34-8c31-b770ec6163c3'
    Method = 'Put'
    Body = @{
        make = "Toyota"
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