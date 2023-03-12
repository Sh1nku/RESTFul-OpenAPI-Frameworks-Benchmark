<?php

namespace App\Filter;

use ApiPlatform\Api\FilterInterface;

class DocumentType implements FilterInterface
{
    public function getDescription(string $resourceClass): array {
        return [
            "document_type" => [
                'property' => NULL,
                'type' => 'int',
                'is_collection' => FALSE,
                'required' => TRUE,
                'description' => "Some example values: <ul><li><code>1</code></li></ul>"
            ],
        ];
    }
}
