#[cfg(test)]
mod tests {
    use image::ImageReader;
    use moseiik::main::{Options, compute_mosaic};

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() -> Result<(), Box<dyn std::error::Error>> {
        let image_path = "/home/eii/QLOG/moseiik/moseiik_test_images/ground-truth-kit.png".to_string();
        let image = ImageReader::open(image_path)?.decode()?.into_rgb8();
        let args = Options {
            image: "/home/eii/QLOG/moseiik/moseiik_test_images/kit.jpeg".to_string(),
            output: "/home/eii/QLOG/moseiik/moseiik_test_images/output.png".to_string(),
            tiles: "/home/eii/QLOG/moseiik/moseiik_test_images/images".to_string(),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 1,
        };
        compute_mosaic(args);
        let result = ImageReader::open("/home/eii/QLOG/moseiik/moseiik_test_images/output.png")?.decode()?.into_rgb8();
        assert_eq!(image, result);
        Ok(())
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() -> Result<(), Box<dyn std::error::Error>> {
        let image_path = "/home/eii/QLOG/moseiik/moseiik_test_images/ground-truth-kit.png".to_string();
        let image = ImageReader::open(image_path)?.decode()?.into_rgb8();
        let args = Options {
            image: "/home/eii/QLOG/moseiik/moseiik_test_images/kit.jpeg".to_string(),
            output: "/home/eii/QLOG/moseiik/moseiik_test_images/output.png".to_string(),
            tiles: "/home/eii/QLOG/moseiik/moseiik_test_images/images".to_string(),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 1,
        };
        compute_mosaic(args);
        let result = ImageReader::open("/home/eii/QLOG/moseiik/moseiik_test_images/output.png")?.decode()?.into_rgb8();
        assert_eq!(image, result);
        Ok(())
    }

    #[test]
    fn test_generic() -> Result<(), Box<dyn std::error::Error>> {
        let image_path = "/home/eii/QLOG/moseiik/moseiik_test_images/ground-truth-kit.png".to_string();
        let image = ImageReader::open(image_path)?.decode()?.into_rgb8();
        let args = Options {
            image: "/home/eii/QLOG/moseiik/moseiik_test_images/kit.jpeg".to_string(),
            output: "/home/eii/QLOG/moseiik/moseiik_test_images/output.png".to_string(),
            tiles: "/home/eii/QLOG/moseiik/moseiik_test_images/images".to_string(),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 1,
        };
        compute_mosaic(args);
        let result = ImageReader::open("/home/eii/QLOG/moseiik/moseiik_test_images/output.png")?.decode()?.into_rgb8();
        assert_eq!(image, result);
        Ok(())
    }
}
