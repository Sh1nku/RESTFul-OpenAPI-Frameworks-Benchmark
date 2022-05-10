from typing import Optional, List

import uvicorn as uvicorn
from fastapi import FastAPI, Query
from fastapi.responses import PlainTextResponse
import httpx
from pydantic.main import BaseModel

client = httpx.AsyncClient()
app = FastAPI(docs_url='/', redoc_url=None)

# host = 'http://127.0.0.1:25900'
host = 'http://varnish'


class SubEntity(BaseModel):
    id: str
    name: str
    number: int


class Entity(BaseModel):
    id: str
    document_type: int
    string_array: List[str]
    int_array: List[int]
    child_objects: Optional[List[SubEntity]] = None


@app.get("/hello_world", summary='Returns Hello World', response_class=PlainTextResponse)
async def hello_world():
    return 'Hello World'


@app.get("/json_serialization", response_model=List[Entity], response_model_exclude_none=True,
         summary='Serializing  a json document')
async def json_serialization(document_type: int) -> List[Entity]:
    """
    :param document_type: Some example values: <ul><li><code>1</code></li></ul>
    """
    r = await client.get(
        host + '/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:' + str(document_type))
    return r.json()['response']['docs']


@app.get("/anonymization", response_model=List[Entity], response_model_exclude_none=True, summary='Serializing  a json document')
async def anonymization():
    r = await client.get(
        host + '/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1')
    data = r.json()['response']['docs']
    for x in data:
        for y in x['child_objects']:
            if y['number'] < 100:
                y['number'] = 0
    return data


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
