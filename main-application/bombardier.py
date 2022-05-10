import json
import subprocess
import time

import requests


def bombard(url, cfg):
    requests.get(url)
    proc = subprocess.Popen(
        [cfg.get('bombardier', 'BOMBARDIER_EXE'), '-c', cfg.get('bombardier', 'BOMBARDIER_THREADS'), '-d',
         cfg.get('bombardier', 'BOMBARDIER_TIME') + 's','-t', cfg.get('bombardier', 'BOMBARDIER_TIMEOUT') + 's', '--format=json', '--print=r',
         url],
        stdout=subprocess.PIPE)
    result: dict = json.loads(proc.stdout.read().decode('utf-8'))['result']
    not_working = 0
    for value in [value for key, value in result.items() if
                  (key.startswith('req') or key == 'others') and key != 'req2xx']:
        not_working += value
    time.sleep(int(cfg.get('bombardier', 'BOMBARDIER_COOLDOWN')))
    return {
        'rps': result['rps']['mean'],
        'working': int(result['req2xx']),
        'error': not_working
    }