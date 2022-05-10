class Benchmark:
    def __init__(self, name, url, status_codes=[200], test=False):
        self.name = name
        self.url = url
        self.status_codes = status_codes
        self.test = test

class BenchmarkEntry:
    def __init__(self, name, url, language, color):
        self.name = name
        self.url = url
        self.language = language
        self.color = color
