#pragma once

#include <oatpp/web/server/api/ApiController.hpp>
#include <oatpp/core/macro/codegen.hpp>
#include <oatpp/core/macro/component.hpp>
#include <oatpp/network/ConnectionPool.hpp>
#include <oatpp/web/client/HttpRequestExecutor.hpp>
#include <iostream>

#include "../SolrClient.hpp"
#include "../dto/SolrEntityResult.hpp"

//const auto API_URL = "127.0.0.1";
//const auto API_PORT = 25900;
const auto API_URL = "varnish";
const auto API_PORT = 80;

class EntityController : public oatpp::web::server::api::ApiController {
public:
  EntityController(const std::shared_ptr<ObjectMapper>& objectMapper, const std::shared_ptr<SolrClient>& solrClient)
    : oatpp::web::server::api::ApiController(objectMapper), m_solrClient(solrClient)
  {}
public:
    typedef EntityController __ControllerType;
    std::shared_ptr<SolrClient> m_solrClient;
    std::shared_ptr<SolrClient> m_objectMapper;
public:

    static std::shared_ptr<EntityController> createShared(){

        auto serializerConfig = oatpp::parser::json::mapping::Serializer::Config::createShared();
        serializerConfig->includeNullFields = false;

        auto deserializerConfig = oatpp::parser::json::mapping::Deserializer::Config::createShared();
        deserializerConfig->allowUnknownFields = true;

        auto objectMapper = oatpp::parser::json::mapping::ObjectMapper::createShared(serializerConfig, deserializerConfig);

        auto connectionProvider = oatpp::network::tcp::client::ConnectionProvider::createShared({API_URL, API_PORT, oatpp::network::Address::IP_4});
        auto requestExecutor = oatpp::web::client::HttpRequestExecutor::createShared(connectionProvider);
        auto solrClient = SolrClient::createShared(requestExecutor, objectMapper);

        return std::make_shared<EntityController>(objectMapper, solrClient);
    }

    #include OATPP_CODEGEN_BEGIN(ApiController)
    ENDPOINT_INFO(json_serialization) {
        info->summary = "Serializing a json document";
        info->addResponse<Object<Entity>>(Status::CODE_200, "application/json");
        info->addResponse<String>(Status::CODE_400, "text/plain");
        info->queryParams.add<Int32>("document_type").description = "Some example values: <ul><li><code>1</code></li></ul>";
    }
    ENDPOINT_ASYNC("GET", "/json_serialization", json_serialization) {
    ENDPOINT_ASYNC_INIT(json_serialization)

        Action act() override {
            auto document_type = request->getQueryParameter("document_type");
            if (document_type->empty() || std::string(document_type).find_first_not_of("0123456789") != std::string::npos) {
                return _return(controller->createResponse(Status::CODE_400, "document_type must be given and be an int"));
            }
            return controller->m_solrClient->getSolrQueryAsync("performance", "*:*", "id,type,int_array,string_array,child_objects,name,number,[child]", 100, "document_type:"+document_type).callbackTo(&json_serialization::onResponse);
        }

        Action onResponse(const std::shared_ptr<SolrClient::Response>& response) {
            return response->readBodyToDtoAsync<Object<SolrEntityResult>>(controller->getDefaultObjectMapper()).callbackTo(&json_serialization::onDeserialize);
        }

        Action onDeserialize(const Object<SolrEntityResult>& data) {
            return _return(controller->createDtoResponse(Status::CODE_200, data->response->docs));
        }
    };


    ENDPOINT_INFO(anonymization) {
        info->summary = "Serializing a json document";
        info->addResponse<Object<Entity>>(Status::CODE_200, "application/json");
    }
    ENDPOINT_ASYNC("GET", "/anonymization", anonymization) {
    ENDPOINT_ASYNC_INIT(anonymization)

        Action act() override {
            return controller->m_solrClient->getSolrQueryAsync("performance", "*:*", "id,type,int_array,string_array,child_objects,name,number,[child]", 100, "document_type:1").callbackTo(&anonymization::onResponse);
        }

        Action onResponse(const std::shared_ptr<SolrClient::Response>& response) {
            return response->readBodyToDtoAsync<Object<SolrEntityResult>>(controller->getDefaultObjectMapper()).callbackTo(&anonymization::onDeserialize);
        }

        Action onDeserialize(const Object<SolrEntityResult>& data) {
            for(auto& obj : *data->response->docs) {
                for(auto& child : *obj->child_objects) {
                    if(child->number < 100) {
                        child->number = 0;
                    }
                }
            }
            return _return(controller->createDtoResponse(Status::CODE_200, data->response->docs));
        }
    };
    #include OATPP_CODEGEN_END(ApiController)
};
