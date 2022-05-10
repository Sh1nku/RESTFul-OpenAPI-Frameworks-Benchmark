<?php
namespace App\DataProvider;

use ApiPlatform\Core\DataProvider\ContextAwareCollectionDataProviderInterface;
use ApiPlatform\Core\DataProvider\ItemDataProviderInterface;
use ApiPlatform\Core\DataProvider\RestrictedDataProviderInterface;
use ApiPlatform\Core\DataProvider\SerializerAwareDataProviderInterface;
use ApiPlatform\Core\DataProvider\SerializerAwareDataProviderTrait;
use App\Entity\Entity;
use App\Entity\HelloWorldEntity;
use App\Repository\AnonymizationRepository;
use App\Repository\JSONSerializationRepository;
use Symfony\Component\HttpKernel\Exception\NotFoundHttpException;

class EntityDataProvider  implements ContextAwareCollectionDataProviderInterface, RestrictedDataProviderInterface, ItemDataProviderInterface, SerializerAwareDataProviderInterface
{
    use SerializerAwareDataProviderTrait;
    /**
     * @inheritDoc
     */
    public function getCollection(string $resourceClass, string $operationName = NULL, array $context = []) {
        $filters_raw = $context['filters'] ?? [];
        $filters_raw = array_change_key_case($filters_raw, CASE_LOWER);
        switch ($operationName) {
            case 'anonymization':
                $repo = new AnonymizationRepository($this->getSerializer());
                break;
            case 'json_serialization':
                $repo = new JSONSerializationRepository($this->getSerializer());
                break;
            case 'hello_world':
                die('Hello World');
        }
        return $repo->getAll($filters_raw);
    }

    /**
     * @inheritDoc
     */
    public function getItem(string $resourceClass, $id, string $operationName = NULL, array $context = []) {
        throw new NotFoundHttpException();
    }

    public function supports(string $resourceClass, string $operationName = NULL, array $context = []): bool
    {
        return $resourceClass === Entity::class || $resourceClass == HelloWorldEntity::class;
    }
}