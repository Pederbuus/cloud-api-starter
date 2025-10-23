## Installation
* [Java](https://www.java.com/en/download/)
* [Maven](https://maven.apache.org/install.html)

## Commands
Run using the IDE
1. `run main` in JavaApiApplication.java

Or use the command-line
1. To compile: `mvn package`
2. To then run it: `java -jar target/java-api-0.0.1-SNAPSHOT.jar`

Will run on http://localhost:8080/...

The connection to the postgresDB is connected via `jdbc`
The connection string is found at `.\src\main\resources\application.properties`

## Controll flow:
- **Model**
    - Holds the types
1. **Controller**
    - The mapping of URLs, parameters and return types
2. **Service**
    - Handles: "If the vehicle year is less than 2000, mark it as classic"
3. **Repository**
    - Executes the SQL query
