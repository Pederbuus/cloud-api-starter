# Multi-Language API Project

## Run using Docker
1. Requires [Docker Desktop](https://www.docker.com/products/docker-desktop/)
2. In the root-directory run `docker-compose up`
   * Spins up containers for `PostgreSQL` and the API-directories (see `docker-compose.yml`) `port 5432`
   * `.Net` - `http port 5297` - `https port 7215`
   * `Rust` - `port 3000` (is setup for hot reload)

> **_NOTE:_**  For deploying Docker makes sence, but for development running some kind of 'watch' on the project would be better.

## 🎯 Goal
Learn to write in **Rust**, **C#** (.NET), **Java**, and **Python**.  (Initial plan)

### Objectives
1. **API Service (per language)**
   - Implement an API that supports `POST`, `GET`, `DELETE` requests to `Postgres`
   - Comment the code and the overall structure of the program
   - Create README for each language (research standard custom)

2. **Testing**
   - Implement tests for each language.
   - Measure **throughput** and performance. (or some other/better metric)

3. **Cloud Setup**
   - Google - `Cloud Run functions`
   - Deploy as a `docker container`
   - Configure deployment on **Azure**. (unsure what is in Azure)



## 📌 Requirements (evolves)

### API Endpoints
- `GET /vehicle` → list all entries
- `POST /vehicle` → create a note
- `POST /vehicle/query` → create a note
- `GET /vehicle/{id}` → retrieve a single note
- `PUT /vehicle/{id}` → edit a note
- `DELETE /vehicle/{id}` → delete a note


### Utility Endpoints
- `GET /ping` → return pong
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


## TODO
**28-08-2025**

<!-- Easy🟢, Medium🟡, Hard🔴 -->
- ~~Research `docker`~~, Google - `Cloud Run functions`, ...
- ~~Fix "hot-reload" for `Docker`, as it should work, but doesn't...~~
- Write automated tests
- ~~Generate a webpage with the API-description.~~ See`.Net`
- ~~`.Net` - redirect https://[::1]:7215/ to https://[::1]:7215/swagger~~
- `.Net` - http://[::1]:5297/swagger can't load as "NetworkError when attempting to fetch resource. /openapi/v1.json". Don't know if it should do this.
<!-- ~~ abc ~~ -->
