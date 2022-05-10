namespace AspNetCore.Models
{
    public class Entity {
        public string id { get; set; }
        public int document_type { get; set; }
        public string[] string_array { get; set; }
        public int[] int_array { get; set; }
        public SubEntity[] child_objects { get; set; }
    }

    public class SubEntity {
        public string id { get; set; }
        public string name { get; set; }
        public int number { get; set; }
    }
}