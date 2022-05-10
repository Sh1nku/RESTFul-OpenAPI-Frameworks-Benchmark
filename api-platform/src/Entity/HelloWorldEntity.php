<?php

namespace App\Entity;

use ApiPlatform\Core\Annotation\ApiProperty;
use ApiPlatform\Core\Annotation\ApiFilter;
use ApiPlatform\Core\Annotation\ApiResource;

/**
 * Class HelloWorldEntity
 *
 * @package App\HelloWorldEntity
 *
 *
 * @ApiResource(
 *      collectionOperations={
 *         "hello_world"={
 *              "method" = "GET",
 *              "path"="/hello_world",
 *              "openapi_context" = {
 *                  "summary" = "Returns Hello World",
 *                  "tags" = {"Default"},
 *                  "responses" = {
 *                      "200" = {
 *                          "content" = {
 *                              "text/plain" = {
 *                                 "schema" = {
 *                                     "type": "string"
 *                                 }
 *                              }
 *                          }
 *                      }
 *                  }
 *             }
 *         }
 *     },
 *     itemOperations={
 *         "get"={
 *             "path"="/hello_world_entity/{id}",
 *             "openapi_context" = {
 *                  "tags" = {"Default"},
 *                  "summary" = "Not implemented"
 *             }
 *         }
 *     },
 *     attributes={
 *          "pagination_enabled"=false
 *     },
 *     formats={"plain"}
 * )
 */

class HelloWorldEntity
{
    /**
     * @var string $id Identifier for result
     *
     * @ApiProperty(identifier=true)
     */
    public $id;
}