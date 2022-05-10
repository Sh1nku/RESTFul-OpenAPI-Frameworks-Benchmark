import hashlib
import os
import random
import subprocess

import requests

def reset_config(host, port, collection, shards, replicationFactor):
    # Delete collection
    r = requests.get(host + ':' + port + '/solr/admin/collections?action=DELETE&omitHeader=true&name=' + collection)
    if r.status_code not in [400, 200]:
        print(r.json()['exception']['msg'])
        exit(1)
    print('Deleted collection')

    # Delete config
    r = requests.get(host + ':' + port + '/solr/admin/configs?action=DELETE&omitHeader=true&name=' + collection)
    if r.status_code not in [400, 200]:
        print(r.json()['exception']['msg'])
        exit(1)
    print('Deleted config')

    # Upload config
    _upload_config(host, port, collection)
    # Create collection
    _create_collection(host, port, collection, shards, replicationFactor)


def _upload_config(host, port, collection):
    result = subprocess.check_output('(cd solrconfig && zip -r - *) > ' + 'config.zip', shell=True,
                                     stderr=subprocess.STDOUT)
    with open('config.zip', 'rb') as zip_file:
        request = requests.post(host + ':' + port + '/solr/admin/configs?action=UPLOAD&name=' + collection,
                                headers={'Content-Type': 'application/octet-stream'}, data=zip_file).json()
        if request['responseHeader']['status'] != 0:
            print('Could not create config\n    {}'.format(request['error']['msg']))
            exit(1)
    print('Created config')


def _create_collection(host, port, collection, shards, replicationFactor):
    request = requests.get(host + ':' + port + '/solr/admin/collections?action=CREATE&name=' + collection +
                           '&numShards={}&replicationFactor={}&collection.configName={}'.format(shards,
                                                                                                replicationFactor,
                                                                                                collection)).json()
    if request['responseHeader']['status'] != 0:
        print('Could not create collection\n    {}'.format(request['error']['msg']))
        exit(1)
    print('Created collection')


def add_data(host, port, collection):
    data = []
    for i in range(100):
        key = str(i)
        obj = {
            'id': key,
            'document_type': 1,
            'int_array': [random.randrange(10) for x in range(10)],
            'string_array': ['%032x' % +random.getrandbits(128) for x in range(10)],
            'child_objects': []
        }
        for k in range(10):
            obj['child_objects'].append(
                {
                    'id': key + '_' + str(k),
                    'name': '%032x' % +random.getrandbits(128),
                    'number': random.randrange(0, 1000)
                }
            )
        data.append(obj)
    r = requests.post('{}:{}/solr/{}/update?commitWithin=1000&overwrite=true&wt=json'.format(host, port, collection),
                    headers={'Content-Type': 'application/json'}, json=data)
    if r.status_code != 200:
        print(r.json()['exception']['msg'])
        exit(1)
    print('Added data')
