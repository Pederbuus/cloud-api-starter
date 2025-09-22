using Microsoft.EntityFrameworkCore;
using Npgsql;
using TodoApi.Models;


var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

builder.Services.AddControllers();
// Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
builder.Services.AddOpenApi();
builder.Services.AddDbContext<TodoContext>(opt =>
    opt.UseInMemoryDatabase("TodoList"));
    
// PostgreSQL, gets ConnectionStrings{DefaultConnection} from appsettings.json
var connectionString = builder.Configuration.GetConnectionString("DefaultConnection");
builder.Services.AddDbContext<VehicleContext>(options =>
    options.UseNpgsql(connectionString));

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
    app.UseSwaggerUi(options =>
    {
        options.DocumentPath = "/openapi/v1.json";
    });
}

app.UseHttpsRedirection();

app.UseAuthorization();

app.MapControllers();


// Apply database migrations on startup
try
{
    // would be used to setup the database initially
    // This project does it the other way, using `docker-entrypoint-initdb.d` to initialize the database
    using (var scope = app.Services.CreateScope())
    {
        var dbContext = scope.ServiceProvider.GetRequiredService<VehicleContext>();
        //dbContext.Database.Migrate();
    }
}
catch (NpgsqlException ex)
{
    Console.WriteLine("Error connecting to the database. Make sure the database is running and the connection string is correct.");
    Console.WriteLine(ex.Message);
}
catch (Exception ex)
{
    Console.WriteLine("An error occurred during database migration. ");
    Console.WriteLine(ex.Message);
}

app.Run();
