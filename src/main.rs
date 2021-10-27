use opencv::core::{CV_PI, Point, Scalar, Vector};
use opencv::imgcodecs::{imread, imwrite, IMREAD_GRAYSCALE, IMREAD_COLOR};
use opencv::imgproc::{canny, hough_lines_p, line};
use opencv::types::{VectorOfVec4i};

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
    let mut edge_img = src_img.clone();
    let result_find_edge = canny(&src_img, &mut edge_img, 100.0, 100.0, 3, false);
    match result_find_edge {
        Ok(_) => imwrite("edge.png", &edge_img, &Vector::new()).ok(),
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    // ハフ変換による直線検出
    let mut lines= VectorOfVec4i::default();
    let result_hough_lines = hough_lines_p(&edge_img, &mut lines, 1.0, CV_PI / 180.0, 50, 0.0, 1000.0);
    match result_hough_lines {
        Ok(_) => {
            // 線分を描画する
            for line_vec in lines.to_vec() {
                line(&mut output_img,
                     Point::new(line_vec[0], line_vec[1]),
                     Point::new(line_vec[2], line_vec[3]),
                     Scalar::new(0.0, 0.0, 255.0, 1.0), 5, 0, 0).ok();
            }
        },
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    }

    imwrite("line.png", &output_img, &Vector::new());
}
