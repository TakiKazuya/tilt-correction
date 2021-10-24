use opencv::core::Vector;
use opencv::imgcodecs::{imread, imwrite, IMREAD_GRAYSCALE, IMREAD_COLOR};
use opencv::imgproc::canny;

const SOURCE_IMAGE_PATH: &str = "src_img.png";

fn main() {
    // 処理元の画像を定義
    let src_img;
    let result_read_img = imread(SOURCE_IMAGE_PATH, IMREAD_GRAYSCALE);
    match result_read_img {
        Ok(img) => src_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    // 出力先の画像を定義
    let mut output_img;
    let result_read_img = imread(SOURCE_IMAGE_PATH, IMREAD_COLOR);
    match result_read_img {
        Ok(img) => output_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    // エッジ検出
    let result_find_edge = canny(&src_img, &mut output_img, 100.0, 100.0, 3, false);
    match result_find_edge {
        Ok(_) => imwrite("edge.png", &output_img, &Vector::new()).ok(),
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };
}
