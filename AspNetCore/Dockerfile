FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /app
COPY AspNetCore.sln ./
COPY AspNetCore/appsettings.json ./AspNetCore/
COPY AspNetCore/AspNetCore.csproj ./AspNetCore/
COPY AspNetCore/Program.cs ./AspNetCore/
COPY AspNetCore/Controllers ./AspNetCore/Controllers
COPY AspNetCore/Models ./AspNetCore/Models
COPY AspNetCore/Services ./AspNetCore/Services
RUN dotnet publish -c Release -o out

FROM mcr.microsoft.com/dotnet/aspnet:6.0 AS runtime
WORKDIR /app

# Full PGO
ENV DOTNET_TieredPGO 1 
ENV DOTNET_TC_QuickJitForLoops 1 
ENV DOTNET_ReadyToRun 0

COPY --from=build /app/out ./
EXPOSE 80
ENTRYPOINT ["dotnet", "AspNetCore.dll"]
