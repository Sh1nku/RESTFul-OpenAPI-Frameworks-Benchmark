namespace AspNetCore.Models
{
    public partial class SolrEntityResult {
        public Response response;
    }

    public partial class Response {
        public Entity[] docs;
    }
}