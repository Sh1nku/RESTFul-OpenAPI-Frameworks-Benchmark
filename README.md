# Benchmarking API-Web servers
This benchmark aims to compare different RESTFul API frameworks and their performance in retrieving data from an external API (in our case Solr) <br>
The following are requirements for a framework being allowed in the benchmarks
* Generate an OpenAPI document by annotating code
  * Summary for route
  * Query parameter description
  * Response type example, and multiple responses
* Validate inputs (either manually or automatically)
* JSON Serialization with "ignore null keys"

I have three benchmarks
* Hello World: Tests the framework overhead with minimal output
* JSON Serialization: Tests the framework in delivering a large JSON document
* Anonymization: Tests taking a large document, and iterating through, anonymizing data if a requirement is met

I also wrote a [blog post](https://inobstudios.com/Post/8/A%20benchmark%20of%20OpenAPI-Capable%20RESTful%20frameworks) on the frameworks

## Frameworks

For running the benchmarks I used [Bombardier](https://github.com/codesenberg/bombardier) <br>
The different frameworks are as follows


|      Framework       | Language | Notes       | Stars                                                                                            |
|----------------------|----------|-------------|--------------------------------------------------------------------------------------------------|
|Actix                 |Rust      | Using the Paperclip plugin for OpenAPI | ![GitHub Repo stars](https://img.shields.io/github/stars/actix/actix?style=social)               |
|Rocket                |Rust      | Using okapi for OpenAPI | ![Github Repo stars](https://img.shields.io/github/stars/SergioBenitez/Rocket?style=social)      | 
|Asp.Net Core          |C#        | Using Swashbuckle for OpenAPI          | ![GitHub Repo stars](https://img.shields.io/github/stars/dotnet/aspnetcore?style=social)         |
|Oat++                 |C++       | | ![GitHub Repo stars](https://img.shields.io/github/stars/oatpp/oatpp?style=social)               
|Jooby                 |Java      | Using Jackson for JSON serialization and OpenAPI plugin for OpenAPI  | ![GitHub Repo stars](https://img.shields.io/github/stars/jooby-project/jooby?style=social)       |
|NestJS-Fastify        |Typescript| | ![GitHub Repo stars](https://img.shields.io/github/stars/nestjs/nest?style=social)               
|FastAPI               |Python    |             | ![GitHub Repo stars](https://img.shields.io/github/stars/nestjs/nest?style=social)               |
|Flask-Restx           |Python    | Using Restx for OpenAPI                | ![GitHub Repo stars](https://img.shields.io/github/stars/pallets/flask?style=social)             
|API Platform Nginx-FPM|PHP       | The framework had to be brutalized to get the "Hello World" benchmark to work | ![GitHub Repo stars](https://img.shields.io/github/stars/api-platform/api-platform?style=social) |
|API Platform Apache   |PHP       | ^ | ![GitHub Repo stars](https://img.shields.io/github/stars/api-platform/api-platform?style=social) |

## Run
`docker-compose up --build --force-recreate` runs the benchmark. <br>
**Warning: the benchmark uses cpu and memory limiting which docker-compose 3x written in Golang does not as of this time yet support. You therefore need to use docker-compose 2x**

Since this benchmark tries to compare the overhead each framework experiences, all requests are done towards varnish to allow for minimal external influence.
Each container is given 4 GB ram, and 4 CPU cores.

## Results
* CPU: AMD Ryzen Threadripper 1950X 16-Core Processor
* Memory: 128GB DDR4
* Storage: M.2 Nvme SSD
# Hello World
| Framework              | Language   |Requests per second|Percent|
|------------------------|------------|------------------:|------:|
| Actix                  | Rust       |             214535|  100.0|
| Oat++                  | C++        |             107975|   50.3|
| Asp.Net Core           | C#         |              86259|   40.2|
| Jooby                  | Java       |              84680|   39.5|
| Rocket                 | Rust       |              59185|   27.6|
| NestJS-Fastify         | Typescript |              17172|    8.0|
| Flask-Restx            | Python     |               2477|    1.2|
| FastAPI                | Python     |               2285|    1.1|
| API Platform Nginx-FPM | PHP        |               1172|    0.5|
| API Platform Apache    | PHP        |               1078|    0.5|
### JSON Serialization
| Framework              | Language   |Requests per second|Percent|
|------------------------|------------|------------------:|------:|
| Actix                  | Rust       |               2271|  100.0|
| Rocket                 | Rust       |               1470|   64.7|
| Jooby                  | Java       |               1106|   48.7|
| Asp.Net Core           | C#         |                848|   37.3|
| Oat++                  | C++        |                423|   18.6|
| NestJS-Fastify         | Typescript |                175|    7.7|
| Flask-Restx            | Python     |                 72|    3.2|
| FastAPI                | Python     |                 54|    2.4|
| API Platform Nginx-FPM | PHP        |                 33|    1.5|
| API Platform Apache    | PHP        |                 31|    1.4|
### Anonymization
| Framework              | Language   |Requests per second|Percent|
|------------------------|------------|------------------:|------:|
| Actix                  | Rust       |               2271|  100.0|
| Rocket                 | Rust       |               1456|   64.1|
| Jooby                  | Java       |               1266|   55.7|
| Asp.Net Core           | C#         |                917|   40.4|
| Oat++                  | C++        |                421|   18.5|
| NestJS-Fastify         | Typescript |                177|    7.8|
| Flask-Restx            | Python     |                 72|    3.2|
| FastAPI                | Python     |                 52|    2.3|
| API Platform Nginx-FPM | PHP        |                 33|    1.5|
| API Platform Apache    | PHP        |                 31|    1.4|