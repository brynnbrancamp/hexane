#define MAX_LUMINOSITY_LEVELS 100

decl_buffer(
	Luminosity,
	{
		u32 lum;
		f32 exposure;
		f32 target_exposure;
	}
)
