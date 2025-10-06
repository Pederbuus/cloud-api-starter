# Create a new vehicle using: Json
# Using the wrong fields fails the whole request

# Ports
$dotnet = 7215 
$rust = 3000
$java = 8080
# $python = 5000?

$Port = $dotnet

# Invoke a the call stored in Params

# GET /vehicle → list all entries
$Params = @{
    Uri = "http://localhost:$Port/vehicle"
    Method = 'Get'
}
Invoke-RestMethod @Params

# GET /vehicle/{id} → retrieve a single note
$Params = @{
    Uri = "http://localhost:$Port/vehicle/00000000-0000-0000-0000-000000000000"
    Method = 'Get'
}

# POST /vehicle → create a note
$Params = @{
    Uri = "http://localhost:$Port/vehicle"
    Method = 'Post'
    Body = @{
        make = "Post1"
        model = "Post1"
        year = (Get-Date).Year
    } | ConvertTo-Json
    ContentType = 'application/json'
}
# POST /vehicle/query → create a note
$Params = @{
    Uri = "http://localhost:$Port/vehicle/query?make=Tesla&model=Model S&year=2020" #&name=John&email=john@example.com'
    Method = 'Post'
}

# PUT /vehicle/{id} → edit a note
$Params = @{
    Uri = "http://localhost:$Port/vehicle/00000000-0000-0000-0000-000000000001"
    Method = 'Put'
    Body = @{
        make = "Put1"
        model = "Put1"
        year = 2020
    } | ConvertTo-Json
    ContentType = 'application/json'
}

# DELETE /vehicle/{id} → delete a note
$Params = @{
    Uri = "http://localhost:$Port/vehicle/00000000-0000-0000-0000-000000000002"
    Method = 'Delete'
}


#------------------------------------------------------------------------------------------------------------------
# Example of capturing the full response object to get status code and content
#------------------------------------------------------------------------------------------------------------------
# PUT
# 1. Define the parameters
$Params = @{
    Uri = "http://localhost:$Port/vehicle/00000000-0000-0000-0000-000000000000"
    Method = 'Put'
    Headers = @{ 'Accept' = 'application/json' }
    Body = @{
        make = "Toyota1"
        model = "Corolla"
        year = 2024
    } | ConvertTo-Json
    ContentType = 'application/json'
}

# 2. Use Invoke-WebRequest and save the entire response object
$Response = Invoke-WebRequest @Params

# 3. Access the status code property
Write-Host "HTTP Status Code: $($Response.StatusCode)"

# 4. Access the content (JSON body)
# NOTE: The Content property is a string, so you must convert it back to an object
$ResponseObject = $Response.Content | ConvertFrom-Json
$ResponseObject

#------------------------------------------------------------------------------------------------------------------
# GET /vehicle/{id} PASS
$Params = @{
    Uri = "http://localhost:$Port/vehicle/00000000-0000-0000-0000-000000000000"
    Method = 'Get'
}
# Exstract status code and content
$Response = Invoke-WebRequest @Params
Write-Host "HTTP Status Code: $($Response.StatusCode)"
$ResponseObject = $Response.Content | ConvertFrom-Json
$ResponseObject

# GET /vehicle/{id} FAIL
$Params = @{
    Uri = "http://localhost:$Port/vehicle/00000000-0000-0000-0000-111111111111"
    Method = 'Get'
}
# Exstract status code and content
$Response = Invoke-WebRequest @Params
Write-Host "HTTP Status Code: $($Response.StatusCode)"
$ResponseObject = $Response.Content | ConvertFrom-Json
$ResponseObject

#------------------------------------------------------------------------------------------------------------------

$Params = @{
    Uri = "http://localhost:$Port/vehicle"
    Method = 'POST'
    Headers = @{ 'Accept' = 'application/json' }
    Body = @{
        make = "New Make"
        model = "New Model"
        year = 2025
    } | ConvertTo-Json
    ContentType = 'application/json'
}

Invoke-RestMethod @Params