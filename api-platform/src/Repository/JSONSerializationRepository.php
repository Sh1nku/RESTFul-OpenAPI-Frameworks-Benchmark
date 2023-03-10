<?php

namespace App\Repository;

use App\Entity\SolrResponse;
use Symfony\Component\HttpFoundation\Exception\BadRequestException;

class JSONSerializationRepository extends BaseRepository
{
    public function get($filters)
    {
        if (!is_numeric($filters['document_type'])) {
            throw new BadRequestException('document_type must be an int');
        }
        $ch = curl_init(self::URL.'/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:'.$filters['document_type']);
        curl_setopt($ch,CURLOPT_POST, true);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        return $this->serializer->deserialize(curl_exec($ch),SolrResponse::class, 'json')->response->docs;
    }
}
