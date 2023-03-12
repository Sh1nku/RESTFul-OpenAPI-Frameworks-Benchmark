<?php

namespace App\Entity;

use ApiPlatform\Metadata\ApiProperty;
use ApiPlatform\Metadata\ApiResource;
use ApiPlatform\Metadata\GetCollection;
use App\State\DefaultProvider;

#[ApiResource(
    formats: ['plain' => ['text/plain']],
    normalizationContext: ['skip_null_values' => false],
    paginationEnabled: false
)]
#[GetCollection(
    uriTemplate: '/hello_world',
    openapiContext: [
        'summary' => 'Returns Hello World',
        'tags' => ['Default'],
        'responses' => [
            "200" => [
                'content' => [
                    'text/plain' => [
                        'schema' => [
                            'type' => 'string'
                        ]
                    ]
                ]
            ]
        ]
    ],
    provider: DefaultProvider::class
)]
class HelloWorldEntity
{
    #[ApiProperty(identifier: true)]
    public string $id;
}
