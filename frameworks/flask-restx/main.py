#!/usr/bin/env python3

import logging
import sys

import requests
from flask import Flask
from flask_restx import Api, Resource, fields, reqparse
from werkzeug.middleware.proxy_fix import ProxyFix

app = Flask(__name__)
app.config['RESTX_MASK_SWAGGER'] = False
log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)
app.wsgi_app = ProxyFix(app.wsgi_app)
api = Api(app, version='1.0', title='Flask-Restx API')

host = 'http://varnish'
# host = 'http://localhost:25900'

sub_entity = api.model('SubEntity', {
    'id': fields.String,
    'name': fields.String,
    'number': fields.String
})

entity = api.model('Entity', {
    'id': fields.String,
    'document_type': fields.Integer,
    'string_array': fields.List(fields.String),
    'int_array': fields.List(fields.Integer),
    'child_objects': fields.List(fields.Nested(sub_entity))
})


@api.route('/hello_world')
class HelloWorld(Resource):
    @api.produces(['text/plain'])
    @api.response(200, 'Success', str)
    def get(self):
        """Returns Hello World"""
        return 'Hello World'


json_serialization_parser = reqparse.RequestParser()
json_serialization_parser.add_argument("document_type", type=int,
                                       required=True)


@api.route('/json_serialization')
class JSONSerialization(Resource):
    @api.expect(json_serialization_parser)
    @api.response(400, 'Validation Error')
    @api.marshal_with(entity, skip_none=True)
    def get(self):
        """Serializing  a json document"""
        args = json_serialization_parser.parse_args()
        r = requests.get(
            host + '/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:' + str(
                args['document_type']))
        return r.json()['response']['docs']


@api.route('/anonymization')
class Anonymization(Resource):
    @api.marshal_with(entity, skip_none=True)
    def get(self):
        """Serializing  a json document"""
        r = requests.get(
            host + '/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1')
        data = r.json()['response']['docs']
        for x in data:
            for y in x['child_objects']:
                if y['number'] < 100:
                    y['number'] = 0
        return data


if __name__ == '__main__':
    app.run('0.0.0.0', 8000, debug=True if '--debug' in sys.argv else False)
