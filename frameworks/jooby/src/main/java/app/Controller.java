package app;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.jooby.Context;
import io.jooby.MediaType;
import io.jooby.annotations.*;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.Parameters;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class Controller {
    HttpClient client;
    ObjectMapper object_mapper;
    //final String api_url = "http://localhost:25900";
    final String api_url = "http://varnish";

    public Controller(HttpClient client, ObjectMapper object_mapper) {
        this.client = client;
        this.object_mapper = object_mapper;
    }


    @GET(value = "hello_world")
    @Operation(
            summary = "Returns Hello World"
    )
    @ApiResponse(responseCode = "200", content = {
            @Content(mediaType = MediaType.TEXT, schema = @Schema(type = "string"))
    })
    public String hello_world() {
        return "Hello World";
    }


    @GET(value = "json_serialization")
    @Operation(
            summary = "Serializing  a json document"
    )
    @Parameters({
            @Parameter(description = "Some example values: <ul><li><code>1</code></li></ul>", required = true, name = "document_type")
    })
    @ApiResponse(responseCode = "400")
    public Entity[] json_serialization(@QueryParam int document_type) throws JsonProcessingException {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create(api_url+"/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:"+document_type))
                .build();
        var data = client.sendAsync(request, HttpResponse.BodyHandlers.ofString())
                .thenApply(response -> response).join();
        return object_mapper.readValue(data.body(), SolrEntityResult.class).response.docs;
    }


    @GET(value = "anonymization")
    @Operation(
            summary = "Serializing  a json document"
    )
    public Entity[] anonymization() throws JsonProcessingException {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create(api_url+"/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1"))
                .build();
        var data = client.sendAsync(request, HttpResponse.BodyHandlers.ofString())
                .thenApply(response -> response).join();
        var result = object_mapper.readValue(data.body(), SolrEntityResult.class).response.docs;
        for (var document: result) {
            for (var child: document.child_objects) {
                if(child.number < 100) {
                    child.number = 0;
                }
            }
        }
        return result;
    }
}
