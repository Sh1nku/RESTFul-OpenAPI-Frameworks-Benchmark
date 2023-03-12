using System.Threading.Tasks;

namespace AspNetCore.Services
{
    public interface IHttpApiService
    {
        public Task<byte[]> GetDataFromApi(string url);
    }
}