#pragma once

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include "SubEntity.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)



class Entity : public oatpp::DTO {

    DTO_INIT(Entity, DTO)

    DTO_FIELD(String, id);
    DTO_FIELD(Int32, document_type);
    DTO_FIELD(List<String>, string_array);
    DTO_FIELD(List<Int32>, int_array);
    DTO_FIELD(List<Object<SubEntity>>, child_objects);

};

#include OATPP_CODEGEN_END(DTO)