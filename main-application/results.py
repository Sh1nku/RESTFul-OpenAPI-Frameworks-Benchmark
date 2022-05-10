import json
from typing import Dict

from pytablewriter import MarkdownTableWriter
from BenchmarkStructures import BenchmarkEntry


def print_results(results: dict):
    for benchmark, value in results.items():
        if value:
            print(benchmark.upper())
            max_length = max([len(x) for x in value.keys()])
            max_result = max([int(x['rps']) for x in value.values()])
            print('    %-{}s  %-6s  %-5s  %-5s'.format(max_length) % ('Name', 'Requests per second', '%', 'Err%'))
            for type, result in reversed(sorted(value.items(), key=lambda item: int(item[1]['rps']))):
                print('    %-{}s  %-6d  %-5.1f  %-5.1f'.format(max_length) % (
                    type, result['rps'], (float(int(result['rps'])) / float(max_result)) * 100,
                    (result['error'] / (result['working'] + result['error'])) * 100))


def print_markdown(results: dict, frameworks: Dict[str, BenchmarkEntry]):
    for key, value in results.items():
        if value:
            value = {k: v for k, v in sorted(value.items(), key=lambda x: int(x[1]['rps']), reverse=True)}
            matrix = []
            max_result = max([int(x['rps']) for x in value.values()])
            for name, result in value.items():
                matrix.append([name, frameworks[name].language, int(result['rps']), '%.1f' % ((float(int(result['rps'])) / float(max_result)) * 100)])

            writer = MarkdownTableWriter(
                table_name=key,
                headers=['Framework', 'Language', 'Requests per second', 'Percent'],
                value_matrix=matrix,
            )
            writer.write_table()


def print_chartjs(results: dict, frameworks):
    for key, value in results.items():
        if value:
            value = {k: v for k, v in sorted(value.items(), key=lambda x: int(x[1]['rps']), reverse=True)}
            labels = [x for x in value.keys()]
            datasets = [int(x['rps']) for x in value.values()]
            colors = [frameworks[x].color for x in value.keys()]
            result = {
                "type": "bar",
                "data": {
                    "labels": labels,
                    "datasets": [{
                        "data": datasets,
                        "backgroundColor": colors
                    }]
                },
                "options": {
                    "plugins": {
                        "legend": False
                    }
                }
            }
            print(key)
            print(json.dumps(result))
