package com.java.java_api.service;

import com.java.java_api.model.Vehicle;
import com.java.java_api.model.VehicleUpdateDto;
import com.java.java_api.repository.VehicleRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import java.util.List;
import java.util.UUID;

/**
 * Service layer containing business logic for Vehicles.
 * This class orchestrates data access (Repository) and applies business rules.
 */
@Service // Designates this class as a Service component for Spring's container
public class VehicleService {

    // Inject the Repository: The Service layer relies on the Repository for data access
    private final VehicleRepository vehicleRepository;

    // Constructor Injection: Preferred method for injecting dependencies
    @Autowired
    public VehicleService(VehicleRepository vehicleRepository) {
        this.vehicleRepository = vehicleRepository;
    }

    /**
     * Retrieves all vehicles from the database.
     * @return A list of all Vehicle entities.
     */
    public List<Vehicle> findAll() {
        // Example of where business logic might go:
        // System.out.println("Applying filtering and security checks...");
        return vehicleRepository.findAll();
    }

    /**
     * Retrieves a single vehicle by its ID.
     * @param id The ID of the vehicle.
     * @return The Vehicle entity.
     */
    public Vehicle findById(UUID id) {
        // Delegates directly to the repository (in a real app, you might handle 
        // a "not found" exception here before returning it to the controller).
        return vehicleRepository.findById(id); 
    }
    
    /**
     * Saves a new vehicle to the database.
     * @param vehicle The Vehicle object to save.
     * @return The saved Vehicle object (often with a generated ID).
     */
    public Vehicle save(VehicleUpdateDto vehicle) {
        // Example of business logic before saving:
        // if (vehicle.getYear() > 2025) {
        //     throw new IllegalArgumentException("Cannot save vehicle from the future.");
        // }
        Vehicle newVehicle = new Vehicle(vehicle.getMake(), vehicle.getModel(), vehicle.getYear());
        return vehicleRepository.save(newVehicle);
    }

    /**
     * Updates an existing vehicle in the database.
     * @param vehicle The Vehicle object with updated data.
     * @return The updated Vehicle object.
     */
    public Vehicle update(UUID id, VehicleUpdateDto vehicleDto) {
        // Fetch the existing vehicle (in a real app, handle "not found" case)
        Vehicle existingVehicle = vehicleRepository.findById(id);
        // Update fields
        existingVehicle.setMake(vehicleDto.getMake());
        existingVehicle.setModel(vehicleDto.getModel());
        existingVehicle.setYear(vehicleDto.getYear());
        // Save the updated vehicle
        return vehicleRepository.update(existingVehicle);
    }

    /**
     * Deletes a vehicle by its ID.
     * @param id The ID of the vehicle to delete.
     */
    public void deleteById(UUID id) {
        // Delegates to the repository.
        vehicleRepository.deleteById(id);
    }
}
