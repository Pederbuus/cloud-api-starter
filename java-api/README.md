# README Java
`run main` in JavaApiApplication.java

1. To compile: `mvn package`
2. To then run it: `cd target && java -jar java-api-0.0.1-SNAPSHOT.jar`

Will run on http://localhost:8080/...

## Controll flow:
- **Model**
    - Holds the types
1. **Controller**
    - The mapping of URLs, parameters and return types
2. **Service**
    - Handles: "If the vehicle year is less than 2000, mark it as classic"
3. **Repository**
    - Executes the SQL query
