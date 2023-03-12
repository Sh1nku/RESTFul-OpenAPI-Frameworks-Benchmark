<?php

namespace App\State;

use ApiPlatform\Metadata\Operation;
use ApiPlatform\State\ProviderInterface;
use ApiPlatform\State\SerializerAwareProviderInterface;
use ApiPlatform\State\SerializerAwareProviderTrait;
use App\Repository\AnonymizationRepository;
use App\Repository\JSONSerializationRepository;
use Symfony\Component\HttpKernel\Exception\NotFoundHttpException;

final class DefaultProvider implements ProviderInterface, SerializerAwareProviderInterface
{
    use SerializerAwareProviderTrait;

    public function provide(Operation $operation, array $uriVariables = [], array $context = []): object|array|null
    {
        $filters_raw = $context['filters'] ?? [];
        $filters_raw = array_change_key_case($filters_raw, CASE_LOWER);
        if (str_contains($context['request_uri'], '/hello_world')) {
            die('Hello World');
        }
        else if (str_contains($context['request_uri'], '/json_serialization')) {
            $repo = new JSONSerializationRepository($this->getSerializer());
            return $repo->get($filters_raw);
        }
        else if (str_contains($context['request_uri'], '/anonymization')) {
            $repo = new AnonymizationRepository($this->getSerializer());
            return $repo->get($filters_raw);
        }
        throw new NotFoundHttpException();
    }
}
