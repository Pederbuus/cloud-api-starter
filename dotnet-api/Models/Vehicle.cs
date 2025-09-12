using Microsoft.EntityFrameworkCore;

namespace VehicleApi.Models
{
    public class Vehicle
    {
        public Guid id { get; set; }
        public string? make { get; set; }
        public string? model { get; set; }
        public int year { get; set; }

        // MyDbContext will represent a session with the database.
        public class MyDbContext : DbContext
        {
            // Constructor that accepts DbContextOptions.
            // This is how the connection string is passed to the context.
            public MyDbContext(DbContextOptions<MyDbContext> options)
                : base(options)
            {
            }

            // DbSet<T> represents a collection of all entities in the context,
            // or that can be queried from the database.
            public DbSet<Vehicle> Vehicles { get; set; }

            // Table-name must exactly match the name in the database, which is 'vehicle' not 'Vehicles'
            protected override void OnModelCreating(ModelBuilder modelBuilder)
            {
                modelBuilder.Entity<Vehicle>().ToTable("vehicle");
            }
        }
    }
}