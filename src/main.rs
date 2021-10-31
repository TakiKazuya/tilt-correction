use std::collections::HashMap;
use opencv::core::{CV_PI, Point, Scalar, Vector, Size, MatTrait, Point2f, BORDER_CONSTANT};
use opencv::imgcodecs::{imread, imwrite, IMREAD_GRAYSCALE, IMREAD_COLOR};
use opencv::imgproc::{canny, hough_lines_p, line, warp_affine, get_rotation_matrix_2d, WARP_INVERSE_MAP};
use opencv::types::{VectorOfVec4i};
use ang::atan2;

const SOURCE_IMAGE_PATH: &str = "src_img.png";

fn main() {
    // 処理元の画像を定義
    let src_img;
    let result_read_img = imread(SOURCE_IMAGE_PATH, IMREAD_GRAYSCALE);
    match result_read_img {
        Ok(img) => src_img = img,
        Err(code) => {
            panic!("code: {:?}", code);
        }
    };

    // 出力先の画像を定義
    let output_img;
    let result_read_img = imread(SOURCE_IMAGE_PATH, IMREAD_COLOR);
    match result_read_img {
        Ok(img) => output_img = img,
        Err(code) => {
            panic!("code: {:?}", code);
        }
    };

    // エッジ検出
    let mut edge_img = src_img.clone();
    let result_find_edge = canny(&src_img, &mut edge_img, 100.0, 100.0, 3, false);
    match result_find_edge {
        Ok(_) => imwrite("edge.png", &edge_img, &Vector::new()).ok(),
        Err(code) => {
            panic!("code: {:?}", code);
        }
    };

    // ハフ変換による直線検出
    let mut line_img = output_img.clone();
    let mut lines= VectorOfVec4i::default();
    let result_hough_lines= hough_lines_p(&edge_img, &mut lines, 1.0, CV_PI / 180.0, 250, 0.0, 1000.0);
    match result_hough_lines {
        Ok(_) => {
            // 線分を描画する
            for line_vec in lines.to_vec() {
                line(&mut line_img,
                     Point::new(line_vec[0], line_vec[1]),
                     Point::new(line_vec[2], line_vec[3]),
                     Scalar::new(0.0, 0.0, 255.0, 1.0),
                     1,
                     0,
                     0
                ).ok();
            }
            imwrite("line.png", &line_img, &Vector::new()).ok();
        },
        Err(code) => {
            panic!("code: {:?}", code);
        }
    }

    // 線分の角度の配列を作成する
    let mut angles = vec![];
    for line_vec in lines.to_vec() {
        let x1 = line_vec[0] as f64;
        let y1 = line_vec[1] as f64;
        let x2 = line_vec[2] as f64;
        let y2 = line_vec[3] as f64;
        let angle = atan2(y2 - y1, x1 - x2).in_degrees().round() as i32;
        angles.push(angle);
    }

    // 角度の配列から最頻値を取得(複数ある場合は最初の要素を選択)
    let angle = get_mode(&angles).first().unwrap().clone();

    // 角度が0or90の場合は何もしない。
    // それ以外はアフィン変換
    let result_img = if angle.abs() == 0 || angle.abs() == 90 {
        src_img
    } else {
        let mut dst_img = src_img.clone();
        let width = dst_img.cols();
        let height = dst_img.rows();
        let center = Point2f::new((width/2) as f32, (height/2) as f32); // 回転中心
        let rotation_angle = (angle - 180) as f64; // 回転する角度

        let m =
            get_rotation_matrix_2d(center, rotation_angle, 1.0)
                .unwrap_or_else(|code| {
                    panic!("code: {}", code)
                });

        let size = Size::new(width, height); // 出力画像のサイズ
        let result_affine = warp_affine(&src_img, &mut dst_img, &m, size, WARP_INVERSE_MAP, BORDER_CONSTANT, Scalar::default());

        match result_affine {
            Ok(_) => {
                dst_img
            },
            Err(code) => {
                panic!("code: {}", code);
            }
        }
    };

    imwrite("result.png", &result_img, &Vector::new())
        .unwrap_or_else(|e| panic!("code: {}", e));
}

pub fn get_mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}
