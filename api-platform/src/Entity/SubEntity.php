<?php

namespace App\Entity;
use ApiPlatform\Core\Annotation\ApiProperty;

class SubEntity
{
    /**
     * @var string $id Identifier for result
     *
     * @ApiProperty(identifier=true)
     */
    public $id;

    /** @var string $name */
    public $name;

    /** @var int $number */
    public $number;
}