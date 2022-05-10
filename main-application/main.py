#!/usr/bin/env python3

import configparser
from typing import List, Dict

import requests
import time

from solr import reset_config, add_data
from results import print_results, print_chartjs, print_markdown
from bombardier import bombard
from BenchmarkStructures import Benchmark, BenchmarkEntry

cfg = configparser.ConfigParser()
cfg.read('config.conf', encoding='utf-8')
benchmark_entries: List[BenchmarkEntry] = []
benchmarks: List[Benchmark] = [
    Benchmark('Hello World', '/hello_world', [200], False),
    Benchmark('JSON Serialization', '/json_serialization?document_type=1', [200], False),
    Benchmark('Query parameter validation', '/json_serialization?document_type=test', [400, 422], True),
    Benchmark('Anonymization', '/anonymization', [200], False),
]

for key, value in [(x, cfg[x]) for x in cfg.sections() if x.startswith('framework_')]:
    benchmark_entries.append(
        BenchmarkEntry(value.get('NAME'), value.get('URL'), value.get('LANGUAGE'), value.get('COLOR')))

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

print('Verifying benchmarks')
# Verify benchmarks
for benchmark_entry in benchmark_entries:
    for benchmark in benchmarks:
        try:
            passed = True
            r = requests.get(benchmark_entry.url + benchmark.url)
            if r.status_code not in benchmark.status_codes:
                raise Exception()
        except Exception as e:
            passed = False

        print(f'{benchmark_entry.name} | {benchmark.name}: {"Passed" if passed else "Failed"}')

print('Running benchmarks')
# Do benchmarks
benchmark_results = {}
for benchmark in [x for x in benchmarks if not x.test]:
    print(benchmark.name)
    benchmark_results[benchmark.name] = {}
    for benchmark_entry in benchmark_entries:
        print('    ' + benchmark_entry.name)
        benchmark_results[benchmark.name][benchmark_entry.name] = bombard(benchmark_entry.url + benchmark.url, cfg)

# Print
print_results(benchmark_results)
print_chartjs(benchmark_results, {item.name: item for item in benchmark_entries})
print_markdown(benchmark_results, {item.name: item for item in benchmark_entries})
