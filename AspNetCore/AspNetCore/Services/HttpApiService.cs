using System.Net.Http;
using System.Threading.Tasks;

namespace AspNetCore.Services
{
    public class HttpApiService: IHttpApiService
    {
        private readonly HttpClient _httpClient;

        public HttpApiService(HttpClient httpClient)
        {
            _httpClient = httpClient;
        }
        
        public async Task<byte[]> GetDataFromApi(string url)
        {
            var response = await _httpClient
                .GetAsync(url);
            var resultContent = response.Content;
            return await resultContent.ReadAsByteArrayAsync();
        }
    }
}