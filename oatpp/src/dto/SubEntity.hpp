#pragma once

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)



class SubEntity : public oatpp::DTO {

    DTO_INIT(SubEntity, DTO)

    DTO_FIELD(String, id);
    DTO_FIELD(String, name);
    DTO_FIELD(Int32, number);

};

#include OATPP_CODEGEN_END(DTO)