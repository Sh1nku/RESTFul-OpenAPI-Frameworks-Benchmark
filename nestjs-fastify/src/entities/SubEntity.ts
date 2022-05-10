import { ApiProperty } from '@nestjs/swagger';

export class SubEntity {
  @ApiProperty()
  id: string;
  @ApiProperty()
  name: string;
  @ApiProperty()
  number: number;
}
