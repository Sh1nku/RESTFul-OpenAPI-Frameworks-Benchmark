#pragma once

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include "SubEntity.hpp"
#include "Entity.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

class Response : public oatpp::DTO {

    DTO_INIT(Response, DTO)

    DTO_FIELD(List<Object<Entity>>, docs);

};


class SolrEntityResult : public oatpp::DTO {

    DTO_INIT(SolrEntityResult, DTO)

    DTO_FIELD(Object<Response>, response);

};


#include OATPP_CODEGEN_END(DTO)