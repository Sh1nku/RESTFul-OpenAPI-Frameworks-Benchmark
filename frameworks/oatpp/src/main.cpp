#include <oatpp/web/client/RequestExecutor.hpp>
#include <oatpp/web/server/AsyncHttpConnectionHandler.hpp>
#include <oatpp/network/tcp/client/ConnectionProvider.hpp>
#include <oatpp-swagger/AsyncController.hpp>
#include <oatpp/network/Server.hpp>


#include "controllers/HelloWorldController.hpp"
#include "AppComponent.hpp"
#include "controllers/EntityController.hpp"
#include "SolrClient.hpp"

void run() {

    /* Register Components in scope of run() method */
    AppComponent components;

    OATPP_COMPONENT(std::shared_ptr<oatpp::data::mapping::ObjectMapper>, objectMapper);
    OATPP_COMPONENT(std::shared_ptr<oatpp::web::server::HttpRouter>, router);
    auto helloWorldController = HelloWorldController::createShared();
    auto entityController = EntityController::createShared();


    oatpp::web::server::api::Endpoints docEndpoints;
    docEndpoints.append(router->addController(helloWorldController)->getEndpoints());
    docEndpoints.append(router->addController(entityController)->getEndpoints());
    router->addController(oatpp::swagger::AsyncController::createShared(docEndpoints));

    OATPP_COMPONENT(std::shared_ptr<oatpp::network::ConnectionHandler>, serverConnectionHandler);
    OATPP_COMPONENT(std::shared_ptr<oatpp::network::ServerConnectionProvider>, connectionProvider);
    oatpp::network::Server server(connectionProvider, serverConnectionHandler);

    /* Priny info about server port */
    OATPP_LOGI("MyApp", "Server running on port %s", connectionProvider->getProperty("port").getData());

    /* Run server */
    server.run();

}

int main() {
    oatpp::base::Environment::init();
    run();
    oatpp::base::Environment::destroy();

    return 0;

}

