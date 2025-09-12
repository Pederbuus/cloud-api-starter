using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using VehicleApi.Models;
using Npgsql;
using System.Net.Sockets;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

// Add the database context to the service container.
// It reads the connection string from appsettings.json
// and configures EF Core to use PostgreSQL.
builder.Services.AddDbContext<Vehicle.MyDbContext>(options =>
    options.UseNpgsql(builder.Configuration.GetConnectionString("DefaultConnection")));

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI(options =>
    {
        options.SwaggerEndpoint("/swagger/v1/swagger.json", "v1");
        options.RoutePrefix = string.Empty;
    });
}

app.UseHttpsRedirection();

app.UseAuthorization();

app.MapControllers();

// Apply database migrations on startup
try
{
    using (var scope = app.Services.CreateScope())
    {
        var dbContext = scope.ServiceProvider.GetRequiredService<Vehicle.MyDbContext>();
        dbContext.Database.Migrate();
    }
}
catch (NpgsqlException ex)
{
    Console.WriteLine("Error connecting to the database. Make sure the database is running and the connection string is correct.");
    Console.WriteLine(ex.Message);
}
catch (Exception ex)
{
    Console.WriteLine("An error occurred during database migration.");
    Console.WriteLine(ex.Message);
}

app.Run();