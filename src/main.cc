#include "segappend.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
  segappend_status status;

  unsigned long size;
  void *data;
  status = segappend_load_segment("__custom_sect", &data, &size);
  if (status == segappend_ok) {
    printf("custom segment found\n");
    printf("size: %lu\n", size);
    printf("data: %p\n", data);
    printf("cstring: %s\n", (char *)data);

    status = segappend_load_segment("__custom_sect2", &data, &size);
    if (status == segappend_ok) {
      printf("custom segment 2 found\n");
      printf("size: %lu\n", size);
      printf("data: %p\n", data);
      printf("cstring: %s\n", (char *)data);
    }

    return 0;
  }

  if (argc < 5) {
    printf("Usage: %s <input-file> <segment-name> <segment-data-file> "
           "<output-file>\n",
           argv[0]);
    return 1;
  }

  char *binary_path = argv[1];
  char *segment_name = argv[2];
  char *file_path = argv[3];
  char *output_path = argv[4];

  FILE *file = fopen(file_path, "rb");
  if (!file) {
    printf("Could not open file %s\n", file_path);
    return 1;
  }
  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  fseek(file, 0, SEEK_SET);
  char *file_data = (char *)malloc(file_size);
  fread(file_data, 1, file_size, file);
  fclose(file);

  status = segappend_create_segment(binary_path, segment_name, file_data,
                                    file_size, output_path);
  if (status != segappend_ok) {
    printf("Could not create segment %s in %s: %d\n", segment_name, binary_path,
           status);
    return 1;
  }

  printf("Segment %s created in %s\n", segment_name, output_path);

  return 0;
}
