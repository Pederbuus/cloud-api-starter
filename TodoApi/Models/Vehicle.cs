using System;
using System.ComponentModel.DataAnnotations;
using TodoApi.Models;


namespace TodoApi.Models
{
    public class Vehicle
    {
        // Guid === UUID
        public Guid id { get; set; }
        public string? make { get; set; }
        public string? model { get; set; }
        public int year { get; set; }
    }
}