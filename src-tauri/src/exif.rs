use serde::Serialize;
use std::path::Path;

/// 展平后的 EXIF 信息，全部转成字符串，前端直接展示
#[derive(Debug, Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExifInfo {
  pub camera_make: Option<String>,
  pub camera_model: Option<String>,
  pub lens_model: Option<String>,
  pub aperture: Option<String>,
  pub shutter_speed: Option<String>,
  pub iso: Option<String>,
  pub focal_length: Option<String>,
  pub datetime_original: Option<String>,
  pub gps_latitude: Option<String>,
  pub gps_longitude: Option<String>,
  pub orientation: Option<String>,
}

/// 从图片文件读取 EXIF 信息
pub fn read_exif<P: AsRef<Path>>(path: P) -> Result<ExifInfo, String> {
  let file = std::fs::File::open(path.as_ref()).map_err(|e| e.to_string())?;
  let mut buf = std::io::BufReader::new(&file);
  let reader = exif::Reader::new();
  let exif = match reader.read_from_container(&mut buf) {
    Ok(e) => e,
    // 大部分 PNG/BMP 没有 EXIF，直接返回空
    Err(_) => return Ok(ExifInfo::default()),
  };

  let mut info = ExifInfo::default();
  for field in exif.fields() {
    let value = field.display_value().with_unit(&exif).to_string();
    match field.tag {
      exif::Tag::Make => info.camera_make = Some(strip(&value)),
      exif::Tag::Model => info.camera_model = Some(strip(&value)),
      exif::Tag::LensModel => info.lens_model = Some(strip(&value)),
      exif::Tag::FNumber => info.aperture = Some(format!("f/{}", strip_number(&value))),
      exif::Tag::ExposureTime => info.shutter_speed = Some(strip(&value)),
      exif::Tag::PhotographicSensitivity => info.iso = Some(strip(&value)),
      exif::Tag::FocalLength => info.focal_length = Some(strip(&value)),
      exif::Tag::DateTimeOriginal => info.datetime_original = Some(strip(&value)),
      exif::Tag::GPSLatitude => info.gps_latitude = Some(strip(&value)),
      exif::Tag::GPSLongitude => info.gps_longitude = Some(strip(&value)),
      exif::Tag::Orientation => info.orientation = Some(strip(&value)),
      _ => {}
    }
  }
  Ok(info)
}

/// 去掉 exif crate 生成字符串两侧的引号
fn strip(s: &str) -> String {
  s.trim_matches('"').trim().to_string()
}

/// FNumber 类似 "f/2.8"，我们只要数字部分再加前缀
fn strip_number(s: &str) -> String {
  s.trim_matches('"').trim().replace("f/", "")
}
