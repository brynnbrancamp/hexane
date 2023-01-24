#define CHUNK_SIZE 64
#define AXIS_MAX_CHUNKS 4
#define REGION_SIZE 512
#define VIEW_DISTANCE 128

decl_buffer(
	Region,
	{
		ImageId data;
		ImageId reserve;
		ivec3 observer_position;
		ivec3 floating_origin;
		bool dirty;
		bool first;
	}
)

