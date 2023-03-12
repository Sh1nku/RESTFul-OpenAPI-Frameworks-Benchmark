<?php

namespace App\Entity;

use ApiPlatform\Metadata\ApiResource;
use ApiPlatform\Metadata\GetCollection;
use ApiPlatform\Metadata\ApiProperty;
use App\Filter\DocumentType;
use App\State\DefaultProvider;

#[ApiResource(
    formats: 'json',
    normalizationContext: ['skip_null_values' => false],
    paginationEnabled: false
)]
#[GetCollection(
    uriTemplate: '/json_serialization',
    openapiContext: [
        'summary' => 'Serializing  a json document',
        'tags' => ['Default']
    ],
    filters: [
        DocumentType::Class,
    ],
    provider: DefaultProvider::class
)]
#[GetCollection(
    uriTemplate: '/anonymization',
    openapiContext: [
        'summary' => 'Serializing  a json document',
        'tags' => ['Default']
    ],
    filters: [],
    provider: DefaultProvider::class,
)]

class Entity {
    #[ApiProperty(identifier: true)]
    public string $id;
    public int $document_type;
    /** @var string[] $string_array */
    public $string_array;
    /** @var int[] $int_array */
    public $int_array;
    /** @var SubEntity[] $child_objects */
    public $child_objects;
}
