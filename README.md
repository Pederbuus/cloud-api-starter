# Multi-Language API Project

## Run using Docker
1. Requires Docker Desktop
2. In the root-directory run `docker-compose up`
   * Spins up containers for `PostgreSQL` and the API-directories (see `docker-compose.yml`)

## 🎯 Goal
Learn to write in **Rust**, **Java**, and **Python** (maybe **C++**).  (Initial plan)

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



## 📌 Requirements (not final)

### API Endpoints
- `GET /notes` → list all notes
- `POST /notes` → create a note
- `GET /notes/{id}` → retrieve a single note
- `DELETE /notes/{id}` → delete a note

### Utility Endpoints
- `GET /time` → returns the current time  
- `GET /notes/total` → returns the total number of notes in the DB  

### Security
- Implement **credentials-based access** (authentication/authorization). _Research needed_


## ⏳ Timeline
- **21-08-2025** → Setup Rust (prior **Rust** knowledge: ~1%)
- **27/28-08-2025** → Done with basic `axum` (API / listener) and `tokio_postgres` (database) setup


## TODO
**28-08-2025**

<!-- Easy🟢, Medium🟡, Hard🔴 -->
- Research `docker` and Google - `Cloud Run functions`
- Research `cargo-watch` and its use with `docker`
<!-- ~~ abc ~~ -->
