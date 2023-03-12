package app;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.jooby.Jooby;
import io.jooby.OpenAPIModule;
import io.jooby.json.JacksonModule;
import java.net.http.HttpClient;

public class App extends Jooby {

  {
    ObjectMapper mapper = new ObjectMapper();
    mapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
    install(new OpenAPIModule());
    install(new JacksonModule(mapper));

    HttpClient client = HttpClient.newBuilder().build();
    mvc(new Controller(client, mapper));
  }

  public static void main(final String[] args) {
    runApp(args, App::new);
  }

}
