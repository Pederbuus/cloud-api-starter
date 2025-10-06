package com.java.java_api.model;

import java.util.UUID;

/**
 * Represents a Vehicle model (data structure).
 * This belongs in the 'model' package.
 */

public class Vehicle {
    
    private UUID id;
    private String make;
    private String model;
    private int year;
    
    // Constructor
    public Vehicle(String make, String model, int year) {
        this.id = UUID.randomUUID(); // Generate a unique ID
        this.make = make;
        this.model = model;
        this.year = year;
    }

    // Default Constructor (required by Spring/Jackson for serialization)
    public Vehicle() {}

    // Getters and Setters
    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getMake() {
        return make;
    }

    public void setMake(String make) {
        this.make = make;
    }

    public String getModel() {
        return model;
    }

    public void setModel(String model) {
        this.model = model;
    }

    public int getYear() {
        return year;
    }

    public void setYear(int year) {
        this.year = year;
    }
}