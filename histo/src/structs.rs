pub struct UnmergedImage {
    pub image_path_r: String,
    pub image_path_g: String,
    pub image_path_b: String,
    pub image_path_nir: String,
}

impl UnmergedImage {
    pub fn new(
        image_path_r: &str,
        image_path_g: &str,
        image_path_b: &str,
        image_path_nir: &str,
    ) -> Self {
        UnmergedImage {
            image_path_r: image_path_r.to_string(),
            image_path_g: image_path_g.to_string(),
            image_path_b: image_path_b.to_string(),
            image_path_nir: image_path_nir.to_string(),
        }
    }
}
