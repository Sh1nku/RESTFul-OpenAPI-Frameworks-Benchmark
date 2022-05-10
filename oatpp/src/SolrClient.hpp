#pragma once

#include "oatpp/web/client/ApiClient.hpp"
#include "oatpp/core/macro/codegen.hpp"

class SolrClient: public oatpp::web::client::ApiClient {
#include OATPP_CODEGEN_BEGIN(ApiClient)

API_CLIENT_INIT(SolrClient)

    API_CALL("GET", "solr/{collection}/select",
                   getSolrQuery, PATH(String, collection),
                   QUERY(String, q), QUERY(String, fl),
                   QUERY(Int32, rows), QUERY(String, fq))

    API_CALL_ASYNC("GET", "solr/{collection}/select",
                   getSolrQueryAsync, PATH(String, collection),
                   QUERY(String, q), QUERY(String, fl),
                   QUERY(Int32, rows), QUERY(String, fq))

#include OATPP_CODEGEN_END(ApiClient)
};