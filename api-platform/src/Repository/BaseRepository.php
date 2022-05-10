<?php


namespace App\Repository;

use Symfony\Component\Serializer\SerializerInterface;

class BaseRepository
{
    const PROTOCOL = 'http';
    const HOST = 'varnish';
    const PORT = '80';
    //const HOST = 'localhost';
    //const PORT = '25900';
    const CORE = 'performance';
    const URL = self::PROTOCOL.'://'.self::HOST.':'.self::PORT.'/solr/'.self::CORE;

    protected ?SerializerInterface $serializer;
    public function __construct(?SerializerInterface $serializer)
    {
        $this->serializer = $serializer;
    }

}