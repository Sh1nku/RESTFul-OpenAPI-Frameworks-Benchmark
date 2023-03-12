<?php

namespace App\Repository;

use App\Entity\SolrResponse;

class AnonymizationRepository extends BaseRepository
{
    public function get($filters)
    {
        $ch = curl_init(self::URL.'/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1');
        curl_setopt($ch,CURLOPT_POST, true);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        $entities = $this->serializer->deserialize(curl_exec($ch),SolrResponse::class, 'json')->response->docs;
        foreach ($entities as &$entity) {
            foreach($entity->child_objects as &$child) {
                if($child->number < 100) {
                    $child->number = 0;
                }
            }
        }
        return $entities;
    }
}
