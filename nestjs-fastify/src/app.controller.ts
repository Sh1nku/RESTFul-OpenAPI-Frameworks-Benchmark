import { Controller, Get, Query } from '@nestjs/common';
import { AppService } from './app.service';
import {
  ApiBadRequestResponse,
  ApiOkResponse,
  ApiOperation, ApiProduces,
  ApiProperty,
} from '@nestjs/swagger';
import { plainToClass } from 'class-transformer';
import { SolrResponse } from './entities/SolrResponse';
import { Entity } from './entities/Entity';
import { IsNumberString } from 'class-validator';

class GetTypeQuery {
  @ApiProperty({
    description: `Some example values: <ul><li><code>1</code></li></ul>`,
  })
  @IsNumberString()
  document_type: number;
}

@Controller()
export class AppController {
  readonly HOST = 'http://varnish';
  //readonly HOST = 'http://localhost:25900';

  constructor(private readonly appService: AppService) {}

  @Get('hello_world')
  @ApiOperation({ summary: 'Returns Hello World' })
  @ApiOkResponse({
    type: String,
  })
  @ApiProduces("text/plain")
  async helloWorld(): Promise<string> {
    return 'Hello World';
  }

  @Get('json_serialization')
  @ApiOperation({ summary: 'Serializing  a json document' })
  @ApiOkResponse({
    type: Entity,
    isArray: true,
    description: 'Serializing  a json document',
  })
  @ApiBadRequestResponse({ description: 'Bad Request' })
  async json_serialization(@Query() params: GetTypeQuery): Promise<Entity[]> {
    const result = await this.appService.solr_query(
      this.HOST +
        '/solr/performance/select?fl=id,type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:' +
        params.document_type,
    );
    return plainToClass(SolrResponse, result).response.docs;
  }

  @Get('anonymization')
  @ApiOperation({ summary: 'Serializing  a json document' })
  @ApiOkResponse({
    type: Entity,
    isArray: true,
    description: 'Serializing  a json document',
  })
  async anonymization(): Promise<Entity[]> {
    const result = await this.appService.solr_query(
      this.HOST +
        '/solr/performance/select?fl=id,type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1',
    );
    const entities = plainToClass(SolrResponse, result).response.docs;
    for (const entity of entities) {
      for (const child of entity.child_objects) {
        if (child.number < 100) {
          child.number = 0;
        }
      }
    }
    return entities;
  }
}
