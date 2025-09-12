# Multi-Language API Project

## Run using Docker
1. Requires [Docker Desktop](https://www.docker.com/products/docker-desktop/)
2. In the root-directory run `docker-compose up`
   * Spins up containers for `PostgreSQL` and the API-directorie(s) (see `docker-compose.yml`)

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
- `GET /notes` → list all notes
- `POST /notes` → create a note
- `POST /notes/query` → create a note
- `GET /notes/{id}` → retrieve a single note
- `DELETE /notes/{id}` → delete a note

### Utility Endpoints
- `GET /ping` → return pong
- `GET /notes/total` → returns the total number of notes in the DB  

### Security
- Implement **credentials-based access** (authentication/authorization). _Research needed_


## ⏳ Timeline
- **21-08-2025** → Setup Rust (prior **Rust** knowledge: ~1%)
- **27/28-08-2025** → Done with basic `axum` (API / listener) and `tokio_postgres` (database) setup
- **29-08 / 01-09-2025** → Repository can now be run using `Docker`
- **07-09-2025** → Hot-reload now works AND with `Chef` the reload-part went from 26sec to 1sec
- **09-09-2025** → Have imports in a single file (for now), added a query POST-endpoint
- **12-09-2025** → setup of `.NET`, hot-reload, connection to `postgres`


## TODO
**28-08-2025**

<!-- Easy🟢, Medium🟡, Hard🔴 -->
- ~~Research `docker`~~, Google - `Cloud Run functions`, ...
- ~~Fix "hot-reload" for `Docker`, as it should work, but doesn't...~~
- Write automated tests
- Generate a webpage with the API-description
- Generate an [api-overview](https://learn.microsoft.com/en-us/aspnet/core/tutorials/first-web-api?view=aspnetcore-9.0&tabs=visual-studio-code) using .NET
<!-- ~~ abc ~~ -->
