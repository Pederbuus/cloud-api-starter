using Microsoft.EntityFrameworkCore;
using TodoApi.Models;

public class VehicleContext : DbContext
{
    // Constructor that accepts DbContextOptions.
    // This is how the connection string is passed to the context.
    public VehicleContext(DbContextOptions<VehicleContext> options)
        : base(options)
    {
    }

    // DbSet representing the Vehicles table in the database.
    // entrypoint for querying and saving instances in the database.
    public DbSet<Vehicle> Vehicle { get; set; }

    // hardcode that the table name is 'vehicle'
    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        // This line tells EF Core to map the 'Vehicle' class to the 'vehicle' table.
        modelBuilder.Entity<Vehicle>().ToTable("vehicle");
    }
}