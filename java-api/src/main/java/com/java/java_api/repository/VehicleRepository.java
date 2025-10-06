package com.java.java_api.repository;

import com.java.java_api.model.Vehicle;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.stereotype.Repository;

import java.sql.ResultSet;
import java.sql.SQLException;
import java.util.List;
import java.util.UUID;

/**
 * Repository layer for Vehicle data access using JdbcTemplate.
 * This class handles the direct execution of SQL queries.
 */
@Repository
public class VehicleRepository {

    private final JdbcTemplate jdbcTemplate;

    // Use a private inner class or lambda for RowMapper to map SQL results to Vehicle objects
    private static final RowMapper<Vehicle> VEHICLE_ROW_MAPPER = new RowMapper<Vehicle>() {
        @Override
        public Vehicle mapRow(ResultSet rs, int rowNum) throws SQLException {
            // Mapping the ResultSet columns to the Vehicle model properties
            Vehicle vehicle = new Vehicle();
            vehicle.setId((java.util.UUID) rs.getObject("id")); // 'id' column is stored as UUID
            vehicle.setMake(rs.getString("make"));
            vehicle.setModel(rs.getString("model"));
            vehicle.setYear(rs.getInt("year"));
            return vehicle;
        }
    };

    @Autowired
    public VehicleRepository(JdbcTemplate jdbcTemplate) {
        this.jdbcTemplate = jdbcTemplate;
    }

    /**
     * Executes an SQL SELECT query to retrieve all vehicles.
     * @return A list of Vehicle objects mapped from the database rows.
     */
    public List<Vehicle> findAll() {
        final String sql = "SELECT id, make, model, year FROM vehicle";
        // jdbcTemplate.query executes the SQL and uses the RowMapper to transform each row
        return jdbcTemplate.query(sql, VEHICLE_ROW_MAPPER);
    }
    
    // Example of a custom query method for JDBC
    public Vehicle findById(UUID id) {
        final String sql = "SELECT id, make, model, year FROM vehicle WHERE id = ?";
        return jdbcTemplate.queryForObject(sql, VEHICLE_ROW_MAPPER, id);
    }

    public Vehicle save(Vehicle vehicle) {
        final String sql = "INSERT INTO vehicle (id, make, model, year) VALUES (?, ?, ?, ?)";
        jdbcTemplate.update(sql, vehicle.getId(), vehicle.getMake(), vehicle.getModel(), vehicle.getYear());
        return vehicle;
    }

    public Vehicle update(Vehicle vehicle) {
        final String sql = "UPDATE vehicle SET make = ?, model = ?, year = ? WHERE id = ?";
        jdbcTemplate.update(sql, vehicle.getMake(), vehicle.getModel(), vehicle.getYear(), vehicle.getId());
        return vehicle;
    }

    public void deleteById(UUID id) {
        final String sql = "DELETE FROM vehicle WHERE id = ?";
        jdbcTemplate.update(sql, id);
    }
}
