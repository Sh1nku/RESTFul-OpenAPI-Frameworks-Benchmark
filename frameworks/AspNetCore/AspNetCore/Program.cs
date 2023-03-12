using System;
using System.IO;
using System.Reflection;
using AspNetCore.Services;
using Microsoft.OpenApi.Models;
using Utf8Json.AspNetCoreMvcFormatter;
using Utf8Json.Resolvers;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddControllers().AddMvcOptions(option =>
{
    option.OutputFormatters.Clear();
    option.OutputFormatters.Add(new JsonOutputFormatter (StandardResolver.ExcludeNull));
    option.InputFormatters.Clear();
    option.InputFormatters.Add(new JsonInputFormatter ());
});;
builder.Services.AddEndpointsApiExplorer();

builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc("v1", new OpenApiInfo { Title = "ASP.Net Core API", Version = "v1" });
    c.EnableAnnotations();
    var xmlFilename = $"{Assembly.GetExecutingAssembly().GetName().Name}.xml";
    c.IncludeXmlComments(Path.Combine(AppContext.BaseDirectory, xmlFilename));
});
builder.Services.AddHttpClient<IHttpApiService, HttpApiService>();

var app = builder.Build();
app.UseSwagger(options => {
    options.SerializeAsV2 = true;
});
app.UseSwaggerUI(options =>
{
    options.SwaggerEndpoint("/swagger/v1/swagger.json", "v1");
    options.RoutePrefix = string.Empty;
});
app.UseRouting();
app.MapControllers();
app.Run();