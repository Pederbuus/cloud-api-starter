from typing import Union
import uuid
# uuid.uuid4()

from fastapi import FastAPI, HTTPException
import psycopg2
from typing import Annotated

from fastapi import Depends, FastAPI, HTTPException, Query
from sqlmodel import Field, Session, SQLModel, create_engine, select

# Connect to postgres with SQLModel
DATABASE_URL = "postgresql://user:password@localhost:5432/rust_api_db"

engine = create_engine(DATABASE_URL, echo=True)

def get_session():
    with Session(engine) as session:
        yield session

class Vehicle(SQLModel, table=True):
    id: uuid.UUID | None = Field(default=None, primary_key=True)
    make: str = Field(index=True)
    model: str = Field(index=True)
    year: int = Field(index=True)
# class Vehicle:
#    id: uuid.UUID
#    make: str
#    model: str
#    year: int

SessionDep = Annotated[Session, Depends(get_session)]


# Connect to postgres by psycopg2
connection = psycopg2.connect(database="rust_api_db", user="user", password="password", host="localhost", port=5432)

cursor = connection.cursor()

app = FastAPI()

@app.get("/ping", tags=["Health Check"])
def ping():
    return {"message": "pong"}

@app.get("/vehicle/total")
def read_total_vehicle():
    cursor.execute("SELECT COUNT(*) FROM vehicle;")
    record = cursor.fetchone()
    total = record[0] if record else 0
    return {"total_vehicles": total}


@app.get("/vehicle")
def read_vehicle():
    cursor.execute("SELECT * FROM vehicle;")
    record = cursor.fetchall()
    return {"Data from Database": record}


# made with sqlmodel
@app.get("/vehicle/{id}")
def read_vehicle_item(id: uuid.UUID, session: SessionDep) -> Vehicle:
    vehicle = session.get(Vehicle, id)
    if not vehicle:
        raise HTTPException(status_code=404, detail="Vehicle not found")
    return vehicle

# made with psycopg2
# def read_vehicle_item(id: str):
#     vehicle = session.get(Vehicle, id)
#     try:
#         id_uuid = uuid.UUID(id)
#     except (ValueError, AttributeError):
#         raise HTTPException(status_code=400, detail="Invalid UUID")

#     cursor.execute("SELECT * FROM vehicle WHERE id=%s;", (str(id_uuid),))
#     record = cursor.fetchall()
#     row = record[0] if record else None
#     if row is None:
#         raise HTTPException(status_code=404, detail="Vehicle not found")
#     # assuming columns are (id, make, model, year)
#     id_val, make, model, year = row
#     return {"id": str(id_val), "make": make, "model": model, "year": year}

@app.post("/vehicle/query", status_code=201)
def query_vehicle(make: Union[str, None] = None, model: Union[str, None] = None, year: Union[int, None] = None):
    # basic validation
    if make is None or model is None or year is None:
        raise HTTPException(status_code=400, detail="make, model and year are required")

    try:
        vehicle_id = uuid.uuid4()
        cursor.execute(
            "INSERT INTO vehicle (id, make, model, year) VALUES (%s, %s, %s, %s);",
            (str(vehicle_id), make, model, year)
        )
        connection.commit()
    except psycopg2.IntegrityError as e:
        connection.rollback()
        raise HTTPException(status_code=409, detail="Integrity error: " + str(e))
    except psycopg2.Error as e:
        connection.rollback()
        raise HTTPException(status_code=500, detail="Database error")
    return {"id": str(vehicle_id), "message": "Vehicle added successfully"}

@app.put("/vehicle/{id}")
def update_vehicle(id: str, make: Union[str, None] = None, model: Union[str, None] = None, year: Union[int, None] = None):
    try:
        id_uuid = uuid.UUID(id)
    except (ValueError, AttributeError):
        raise HTTPException(status_code=400, detail="Invalid UUID")

    # require at least one field to update
    updates = []
    values = []
    if make is not None:
        updates.append("make = %s")
        values.append(make)
    if model is not None:
        updates.append("model = %s")
        values.append(model)
    if year is not None:
        updates.append("year = %s")
        values.append(year)

    if not updates:
        raise HTTPException(status_code=400, detail="At least one of make, model or year must be provided")

    # final parameter is the id for the WHERE clause
    values.append(str(id_uuid))
    sql = f"UPDATE vehicle SET {', '.join(updates)} WHERE id = %s RETURNING id, make, model, year;"

    try:
        cursor.execute(sql, tuple(values))
        updated = cursor.fetchone()
        if updated is None:
            connection.rollback()
            raise HTTPException(status_code=404, detail="Vehicle not found")
        connection.commit()
    except psycopg2.IntegrityError as e:
        connection.rollback()
        raise HTTPException(status_code=409, detail="Integrity error: " + str(e))
    except psycopg2.Error:
        connection.rollback()
        raise HTTPException(status_code=500, detail="Database error")

    id_val, make_val, model_val, year_val = updated
    return {"id": str(id_val), "make": make_val, "model": model_val, "year": year_val}

@app.delete("/vehicle/{id}")
def delete_vehicle(id: str):
    try:
        id_uuid = uuid.UUID(id)
    except (ValueError, AttributeError):
        raise HTTPException(status_code=400, detail="Invalid UUID")

    try:
        cursor.execute("DELETE FROM vehicle WHERE id = %s RETURNING id;", (str(id_uuid),))
        deleted = cursor.fetchone()
        if deleted is None:
            connection.rollback()
            raise HTTPException(status_code=404, detail="Vehicle not found")
        connection.commit()
    except psycopg2.Error:
        connection.rollback()
        raise HTTPException(status_code=500, detail="Database error")

    return {"id": str(deleted[0]), "message": "Vehicle deleted successfully"}