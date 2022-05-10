import { HttpService, Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  constructor(private httpService: HttpService) {}

  async solr_query(url): Promise<Record<string, unknown>> {
    const result = await this.httpService.get(url).toPromise();
    return result.data;
  }
}
