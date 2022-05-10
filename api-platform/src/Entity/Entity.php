<?php

namespace App\Entity;

use ApiPlatform\Core\Annotation\ApiProperty;
use ApiPlatform\Core\Annotation\ApiFilter;
use ApiPlatform\Core\Annotation\ApiResource;
use App\Filter\DocumentType;

/**
 * Class Entity
 *
 * @package App\Entity
 *
 *
 * @ApiResource(
 *      collectionOperations={
 *         "json_serialization"={
 *              "method" = "GET",
 *              "path"="/json_serialization",
 *              "filters"={
 *                  App\Filter\DocumentType::Class,
 *              },
 *              "openapi_context" = {
 *                  "summary" = "Serializing  a json document",
 *                  "tags" = {"Default"},
 *                  "responses" = {
 *                      "400" = {
 *                          "description" = "Invalid input"
 *                      }
 *                 }
 *              }
 *
 *         },
 *         "anonymization"={
 *              "method" = "GET",
 *              "path"="/anonymization",
 *              "filters"={
 *              },
 *              "openapi_context" = {
 *                  "summary" = "Serializing  a json document",
 *                  "tags" = {"Default"}
 *              }
 *         }
 *     },
 *     itemOperations={
 *         "get"={
 *             "path"="/entity/{id}",
 *             "openapi_context" = {
 *                  "tags" = {"Default"},
 *                  "summary" = "Not implemented"
 *             }
 *         },
 *
 *     },
 *     normalizationContext={
 *         "skip_null_values" = true
 *     },
 *     attributes={
 *          "pagination_enabled"=false
 *     },
 *     formats={"json"}
 * )
 */


class Entity
{
    /**
     * @var string $id Identifier for result
     *
     * @ApiProperty(identifier=true)
     */
    public $id;
    /** @var int $document_type */
    public $document_type;

    /** @var string[] $string_array */
    public $string_array;

    /** @var int[] $int_array */
    public $int_array;

    /** @var SubEntity[] $child_objects */
    public $child_objects;
}

