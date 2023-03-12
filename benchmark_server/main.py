#!/usr/bin/env python3

from solr import reset_config, add_data
import requests
import time

cfg = configparser.ConfigParser()
cfg.read('config.conf', encoding='utf-8')

# Wait for solr
while True:
    try:
        print("Waiting for solr start")
        r = requests.get('{}:{}/solr/admin/collections?action=CLUSTERSTATUS'.format(cfg.get('solr', 'SOLR_HOST'),
                                                                                    cfg.get('solr', 'SOLR_PORT')))
        if r.status_code != 200:
            raise requests.exceptions.ConnectionError
        time.sleep(10)
        reset_config(cfg.get('solr', 'SOLR_HOST'), cfg.get('solr', 'SOLR_PORT'), cfg.get('solr', 'COLLECTION_NAME'),
                     cfg.get('solr', 'COLLECTION_SHARDS'), cfg.get('solr', 'COLLECTION_REPLICATION_FACTOR'))
        add_data(cfg.get('solr', 'SOLR_HOST'), cfg.get('solr', 'SOLR_PORT'), cfg.get('solr', 'COLLECTION_NAME'))
        break
    except requests.exceptions.ConnectionError:
        time.sleep(5)