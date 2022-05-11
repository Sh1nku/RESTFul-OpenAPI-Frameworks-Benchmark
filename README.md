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


|      Framework       | Language | Notes       | Stars |
|----------------------|----------|-------------|-------|
|Actix                 |Rust      | Using the Paperclip plugin for OpenAPI | ![GitHub Repo stars](https://img.shields.io/github/stars/actix/actix?style=social) |
|Asp.Net Core          |C#        | Using Swashbuckle for OpenAPI          | ![GitHub Repo stars](https://img.shields.io/github/stars/dotnet/aspnetcore?style=social) |
|Oat++                 |C++       | | ![GitHub Repo stars](https://img.shields.io/github/stars/oatpp/oatpp?style=social)
|Jooby                 |Java      | Using Jackson for JSON serialization and OpenAPI plugin for OpenAPI  | ![GitHub Repo stars](https://img.shields.io/github/stars/jooby-project/jooby?style=social) |
|NestJS-Fastify        |Typescript| | ![GitHub Repo stars](https://img.shields.io/github/stars/nestjs/nest?style=social)   
|FastAPI               |Python    |             | ![GitHub Repo stars](https://img.shields.io/github/stars/nestjs/nest?style=social) |
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
### Hello World
|      Framework       | Language |Requests per second|Percent|
|----------------------|----------|------------------:|------:|
|Actix                 |Rust      |             215827|  100.0|
|Asp.Net Core          |C#        |              86755|   40.2|
|Oat++                 |C++       |              86166|   39.9|
|Jooby                 |Java      |              48200|   22.3|
|NestJS-Fastify        |Typescript|              17943|    8.3|
|FastAPI               |Python    |               7548|    3.5|
|Flask-Restx           |Python    |               2248|    1.0|
|API Platform Nginx-FPM|PHP       |               1169|    0.5|
|API Platform Apache   |PHP       |               1094|    0.5|
### JSON Serialization
|      Framework       | Language |Requests per second|Percent|
|----------------------|----------|------------------:|------:|
|Actix                 |Rust      |               2288|  100.0|
|Asp.Net Core          |C#        |               1007|   44.0|
|Jooby                 |Java      |                948|   41.4|
|Oat++                 |C++       |                432|   18.9|
|NestJS-Fastify        |Typescript|                178|    7.8|
|Flask-Restx           |Python    |                 75|    3.3|
|FastAPI               |Python    |                 51|    2.2|
|API Platform Nginx-FPM|PHP       |                 33|    1.4|
|API Platform Apache   |PHP       |                 31|    1.4|
### Anonymization
|      Framework       | Language |Requests per second|Percent|
|----------------------|----------|------------------:|------:|
|Actix                 |Rust      |               2219|  100.0|
|Jooby                 |Java      |               1060|   47.8|
|Asp.Net Core          |C#        |                945|   42.6|
|Oat++                 |C++       |                431|   19.4|
|NestJS-Fastify        |Typescript|                179|    8.1|
|Flask-Restx           |Python    |                 74|    3.3|
|FastAPI               |Python    |                 53|    2.4|
|API Platform Nginx-FPM|PHP       |                 33|    1.5|
|API Platform Apache   |PHP       |                 31|    1.4|

