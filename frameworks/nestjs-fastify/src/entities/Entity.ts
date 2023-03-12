import { SubEntity } from './SubEntity';
import { ApiProperty } from '@nestjs/swagger';

export class Entity {
  @ApiProperty()
  id: string;
  @ApiProperty()
  type: number;
  @ApiProperty({ type: [String] })
  string_array: string[];
  @ApiProperty({ type: [Number] })
  int_array: number[];
  @ApiProperty({ type: [SubEntity] })
  child_objects: SubEntity[];
}
