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