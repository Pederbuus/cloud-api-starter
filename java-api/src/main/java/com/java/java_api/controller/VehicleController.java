package com.java.java_api.controller;

import com.java.java_api.model.Vehicle;
import com.java.java_api.model.VehicleUpdateDto;
import com.java.java_api.service.VehicleService;

import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController // Designates this class as a RESTful controller
@RequestMapping("/vehicle") // Base path for all methods in this controller
public class VehicleController {

    private final VehicleService vehicleService; // Dependency on the Service layer

    // Constructor Injection: Spring automatically injects the VehicleService bean
    public VehicleController(VehicleService vehicleService) {
        this.vehicleService = vehicleService;
    }

    // 1. READ (All) - GET /vehicle
    @GetMapping
    public List<Vehicle> getAllVehicles() {
        // Delegates the call to the Service layer to fetch all vehicle from the DB
        return vehicleService.findAll();
    }

    // 2. READ (One) - GET /vehicle/{id}
    @GetMapping("/{id}")
    public Vehicle getVehicleById(@PathVariable UUID id) {
        // Delegates the call to the Service layer to find a vehicle by its ID
        // Note: For production, you'd handle the case where the ID is not found (e.g., throwing a 404 exception)
        return vehicleService.findById(id); 
    }

    // 3. CREATE - POST /vehicle
    @PostMapping
    @ResponseStatus(HttpStatus.CREATED) // Returns HTTP 201 on success
    public Vehicle createVehicle(@RequestBody VehicleUpdateDto vehicle) {
        // @RequestBody converts the incoming JSON/XML body into a Vehicle object
        return vehicleService.save(vehicle);
    }

    // 4. UPDATE - PUT /vehicle/{id}
    @PutMapping("/{id}")
    public Vehicle updateVehicle(@PathVariable UUID id, @RequestBody VehicleUpdateDto vehicleDto) {
        // 1. Pass both the identifying ID and the payload data (DTO) to the service
        return vehicleService.update(id, vehicleDto);
    }

    // 5. DELETE - DELETE /vehicle/{id}
    @DeleteMapping("/{id}")
    @ResponseStatus(HttpStatus.NO_CONTENT) // Returns HTTP 204 on success (no body content)
    public void deleteVehicle(@PathVariable UUID id) {
        // System.out.println("Deleting vehicle with ID: " + id);
        vehicleService.deleteById(id);
    }
}
// ```eof

// ***

// ### Summary of Changes

// | Annotation/Method | Purpose |
// | :--- | :--- |
// | `@RestController` | Marks the class as a component that handles incoming web requests and automatically serializes the return value (e.g., `List<Vehicle>`) into JSON. |
// | `@RequestMapping` | Sets the **base URL path** for all methods in the controller (e.g., `http://localhost:8080/vehicle`). |
// | **Constructor Injection** | The preferred way to get the `VehicleService` dependency. It ensures the controller cannot be created without its required service. |
// | `@GetMapping` | Maps an HTTP **GET** request. |
// | `@PostMapping` | Maps an HTTP **POST** request (used for creation). |
// | `@PutMapping` | Maps an HTTP **PUT** request (used for full updates). |
// | `@DeleteMapping` | Maps an HTTP **DELETE** request. |
// | `@PathVariable` | Extracts a value from the URL path (e.g., the `{id}` portion). |
// | `@RequestBody` | Instructs Spring to deserialize the incoming request body (usually JSON) into the method parameter object (`Vehicle`). |
// | `@ResponseStatus` | Explicitly sets the HTTP status code (e.g., `201 CREATED` or `204 NO CONTENT`). |

// The video link below shows a full example of a Spring Boot CRUD application using the JDBC approach, which is relevant to your current project architecture. [Spring Boot JDBC using JdbcTemplate](https://www.youtube.com/watch?v=Nc9NmS5kEjU).
// http://googleusercontent.com/youtube_content/5