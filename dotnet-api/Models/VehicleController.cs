using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using VehicleApi.Models;

namespace VehicleApi.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class VehiclesController : ControllerBase
    {
        private readonly Vehicle.MyDbContext _context;

        public VehiclesController(Vehicle.MyDbContext context)
        {
            _context = context;
        }

        [HttpGet]
        public async Task<ActionResult<IEnumerable<Vehicle>>> GetAllVehicles()
        {
            return await _context.Vehicles.ToListAsync();
        }

        [HttpGet("isconnected")]
        public async Task<ActionResult<bool>> IsConnected()
        {
            try
            {
                // This is a simple query that will fail if the connection is not working.
                await _context.Vehicles.AnyAsync();
                return Ok(true);
            }
            catch
            {
                return StatusCode(500, false);
            }
        }
    }
}
