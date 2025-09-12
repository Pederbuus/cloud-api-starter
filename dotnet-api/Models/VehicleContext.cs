using VehicleApi.Models;
using Microsoft.EntityFrameworkCore;

namespace VehicleApi.Models;

public class VehicleContext : DbContext
{
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
        //public DbSet<User> Users { get; set; }
        public DbSet<Vehicle> Vehicles { get; set; }
    }
}