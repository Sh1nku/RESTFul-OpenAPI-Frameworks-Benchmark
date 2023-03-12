using AspNetCore.Models;
using AspNetCore.Services;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.ModelBinding;
using Swashbuckle.AspNetCore.Annotations;
using Utf8Json;

namespace AspNetCore.Controllers
{
    [ApiController]
    public class EntityController : ControllerBase
    {
        private IHttpApiService apiService;
        //private const string SOLR_URL = "http://localhost:25900";
        private const string SOLR_URL = "http://varnish";

        public EntityController(IHttpApiService apiService) =>
            this.apiService = apiService;

        /// <summary>
        /// Returns Hello World
        /// </summary>
        [HttpGet]
        [Route("hello_world")]
        [ProducesResponseType( typeof(string), 200)]
        public IActionResult HelloWorld() {
            return Content("Hello World");
        }

        /// <summary>Serializing  a json document</summary>
        /// <param name="document_type">
        /// Some example values: <ul><li><code>1</code></li></ul>
        /// </param>
        [HttpGet]
        [Route("json_serialization")]
        [ProducesResponseType(StatusCodes.Status200OK, Type = typeof(List<Entity>))]
        [ProducesResponseType(StatusCodes.Status400BadRequest)]
        public async Task<IActionResult> JSONSerialization([FromQuery, BindRequired] int document_type) {
            var root = JsonSerializer.Deserialize<SolrEntityResult>(await apiService.GetDataFromApi(
                SOLR_URL+"/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:" +
                document_type));
            return Ok(root.response.docs);
        }



        /// <summary>
        /// Serializing a json document
        /// </summary>
        [HttpGet]
        [Route("anonymization")]
        [ProducesResponseType(typeof(List<Entity>), 200)]
        public async Task<IActionResult> Large_Requests_Reflection() {
            var root = JsonSerializer.Deserialize<SolrEntityResult>(await apiService.GetDataFromApi(
                SOLR_URL+"/solr/performance/select?fl=id,type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1"));
            foreach (var entity in root.response.docs) {
                foreach (var child in entity.child_objects) {
                    if (child.number < 100) {
                        child.number = 0;
                    }
                }
            }
            return Ok(root.response.docs);
        }
    }
}
