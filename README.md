# Multi-Language API Project

## Run using Docker
1. Requires [Docker Desktop](https://www.docker.com/products/docker-desktop/)
2. In the root-directory run `docker-compose up`
   * Spins up containers for `PostgreSQL` and the API-directories (see `docker-compose.yml`) `port 5432`
   * `.Net` - `http port 5297` - `https port 7215`
   * `Rust` - `port 3000` (is setup for hot reload)
   * `Java` - `port 8080`
   * `Python` - `port 8000`
3. Open [https://[::]:7215](https://[::]:7215) for a web-based overview of the API definition

## Other commands
`docker compose up -d` to detatched mode in console \
`docker ps` list all containers currently running \
`docker compose down -v` remove volumes (remove persistent data), used to re-initialize DB \
`docker compose up --build` ["If you change a service's Dockerfile or the contents of its build directory, run docker compose build to rebuild it."](https://docs.docker.com/reference/cli/docker/compose/build/)\
`docker-compose up -d --no-deps --build` build from scratch
> **_NOTE:_**  For deployment of the project Docker makes sence, but for development running it locally with some kind of 'watch' on the project would be better. Is very good to initialize the DB for development.

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

The `vehicle` table in PostgreSQL looks like:

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

<!-- ### API Endpoints
- `GET /vehicle` → list all entries
- `GET /vehicle/{id}` → retrieve a single note
   - Arguments: id -> id, make, model, year
- `POST /vehicle` → create a note
   - Arguments: make, model, year
- `POST /vehicle/query` → create a note
- `PUT /vehicle/{id}` → edit a note
- `DELETE /vehicle/{id}` → delete a note -->

## API Endpoints
- **GET /vehicle** — List all vehicles  
  _Args:_ none \
  _Res:_ `[ {id, make, model, year} ]`

- **GET /vehicle/{id}** — Get vehicle by ID  
  _Args:_ `id` \
  _Res:_ `{id, make, model, year}`

- **POST /vehicle** — Create a vehicle  
  _Args:_ `{make, model, year}` \
  _Res:_ none

<!-- Should not be this way -->
- **POST /vehicle/query** — List all vehicles  
  _Args:_ `http://localhost:$Port/vehicle/query?make=Tesla&model=Model S&year=2020` \
  _Res:_ none

- **PUT /vehicle/{id}** — Get vehicle by ID  
  _Args:_ `{make?, model?, year?}` \
  _Res:_ none

- **DELETE /vehicle/{id}** — Create a vehicle  
  _Args:_ none \
  _Res:_ none

### Utility Endpoints
- `GET /ping` → returns pong
- `GET /vehicle/total` → returns the total number of vehicles in the DB  

### Docks
| API docs    | Http |
| -------- | -------|
|Swagger Python | http://localhost:8000/docs |
|Redoc Python | http://127.0.0.1:8000/redoc |
|Swagger .Net | https://localhost:7215 |

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
- **13-10-2025** → `Python` - Implementation still missing `Docker`


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
- ~~Begin on `Java`~~, ~~`Python`~~ and `Google Functions`/`Azure`
- `Python` - `SQLmodel` fills the terminal (I don't want it to)
- `Python` - Solve the error:`"GET /favicon.ico HTTP/1.1" 404` after every request
- `Python` - Describe the use of `SQLmodel` and `psycopg2`
- `Python` - Setup `Docker`
- `Rust` & `Python` - DB connection must/should have a client that opens a connection, makes and sends the request, revices the responce and then closes the connection. (Is handled in .Net and Java by frameworks)
- Dobble check that that all of the endpoints are avalible for every language/port (Java is missing the util endpoints)
<!-- ~~ abc ~~ -->
