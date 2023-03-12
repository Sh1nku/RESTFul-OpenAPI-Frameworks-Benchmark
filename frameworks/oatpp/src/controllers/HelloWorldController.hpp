#pragma once

#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/macro/component.hpp"


class HelloWorldController : public oatpp::web::server::api::ApiController {
public:
  HelloWorldController(OATPP_COMPONENT(std::shared_ptr<ObjectMapper>, objectMapper))
          : oatpp::web::server::api::ApiController(objectMapper)
  {}
public:
    static std::shared_ptr<HelloWorldController> createShared(OATPP_COMPONENT(std::shared_ptr<ObjectMapper>,
                                                                      objectMapper)){
        return std::shared_ptr<HelloWorldController>(new HelloWorldController(objectMapper));
    }

    #include OATPP_CODEGEN_BEGIN(ApiController)
    ENDPOINT_INFO(hello_world) {
        info->summary = "Returns Hello World";
        info->addResponse<String>(Status::CODE_200, "text/plain");
    }
    ENDPOINT_ASYNC("GET", "/hello_world", hello_world) {
        ENDPOINT_ASYNC_INIT(hello_world)
        Action act() override {
            auto response = controller->createResponse(Status::CODE_200, "Hello World");
            response->putHeader(Header::CONTENT_TYPE, "text/plain");
            return _return(response);
        }
    };

    ENDPOINT_INFO(ui) {
        info->hide = true;
    }
    ENDPOINT_ASYNC("GET", "/", ui) {
        ENDPOINT_ASYNC_INIT(ui)
        Action act() override {
            auto response = controller->createResponse(Status::CODE_302, "");
            response->putHeader("Location", "/swagger/ui");
            return _return(response);
        }
    };
    #include OATPP_CODEGEN_END(ApiController)
  
};

