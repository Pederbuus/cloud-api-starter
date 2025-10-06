# Multi-Language API Project

## Run using Docker
1. Requires [Docker Desktop](https://www.docker.com/products/docker-desktop/)
2. In the root-directory run `docker-compose up`
   * Spins up containers for `PostgreSQL` and the API-directories (see `docker-compose.yml`) `port 5432`
   * `.Net` - `http port 5297` - `https port 7215`
   * `Rust` - `port 3000` (is setup for hot reload)
   * `Java` - `port 8080`
3. Open [https://[::]:7215](https://[::]:7215) for the API's definition

## Other commands
`docker compose up -d` to detatched mode \
`docker ps` list all containers currently running \
`docker compose down -v` remove volumes (useful but unsure what it does) \
`docker compose up --build`\
> **_NOTE:_**  For deployment of the project Docker makes sence, but for development running it locally with some kind of 'watch' on the project would be better.

## 🎯 Goal
Learn to write in **Rust**, **C#** (.NET), **Java**, and **Python**.

### Objectives
1. **API Service (per language)**
   - Implement an API that supports `POST`, `GET`, `DELETE` and `PUT` requests to `PostgresSQL`
   - Comment the code and the overall structure of the program
   - Create README for each language (research standard custom)

2. **Testing**
   - Implement tests for each language.
   - Measure **throughput** and performance. (or some other/better metric)

3. **Cloud Setup**
   - Google - `Cloud Run functions`
   - Deploy as a `Docker container`
   - Configure deployment on **Azure**. (unsure what is in Azure)



## 📌 Requirements (evolves)

### Example Data Model

A typical `vehicle` table in PostgreSQL could look like:

```sql
CREATE TABLE vehicle (
   id UUID PRIMARY KEY,
   make VARCHAR(255) NOT NULL,
   model VARCHAR(255) NOT NULL,
   year INTEGER NOT NULL CHECK (year >= 0)
);
```

Equivalent C# class:

```csharp
class Vehicle {
   Guid id { get; set; }
   string make { get; set; }
   string model { get; set; }
   int year { get; set; }
}
```

### API Endpoints
- `GET /vehicle` → list all entries
- `GET /vehicle/{id}` → retrieve a single note
- `POST /vehicle` → create a note
- `POST /vehicle/query` → create a note
- `PUT /vehicle/{id}` → edit a note
- `DELETE /vehicle/{id}` → delete a note


### Utility Endpoints
- `GET /ping` → returns pong
- `GET /vehicle/total` → returns the total number of vehicles in the DB  

### Security
- Implement **credentials-based access** (authentication/authorization). _Research needed_


## ⏳ Timeline
- **21-08-2025** → Setup Rust (prior **Rust** knowledge: ~1%)
- **27/28-08-2025** → Done with basic `axum` (API / listener) and `tokio_postgres` (database) setup
- **29-08 / 01-09-2025** → Repository can now be run using `Docker`
- **07-09-2025** → Hot-reload now works AND with `Chef` the reload-part went from 26sec to 1sec
- **09-09-2025** → Have imports in a single file (for now), added a query POST-endpoint
- **12-09-2025** → setup of `.NET`, hot-reload, connection to `postgres`
- **17-09-2025** → Redone `.NET`, no hot-reload, connection to `postgres`
- **22-09-2025** → Comment `.NET`
- **29-09-2025** → `Rust` - Fix logic error, implement `Put`, write a few tests (more are needed)
- **06-10-2025** → `Java` - Implementation + `Rust` `Delete`


## TODO
<!-- Easy🟢, Medium🟡, Hard🔴 -->
- ~~Research `docker`~~, Google - `Cloud Run functions`, ...
- ~~Fix "hot-reload" for `Docker`, as it should work, but doesn't...~~
- Write automated tests
- ~~Generate a webpage with the API-description.~~ See `.Net`
- ~~`.Net` - redirect https://[::1]:7215/ to https://[::1]:7215/swagger~~
- `.Net` - http://[::1]:5297/swagger can't load as "NetworkError when attempting to fetch resource. /openapi/v1.json". Don't know if it should do this.
- `Rust` & `.Net` - return httpStatus for every request?
- `.Net` - Catch the non-Nullable before all of the errors
- ~~Begin on `Java`~~, `Python` and `Google Functions`/`Azure`
<!-- ~~ abc ~~ -->
