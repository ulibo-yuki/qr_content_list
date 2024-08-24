use image::Luma;
use log::info;
use qrcode::QrCode;
use std::path::PathBuf;

pub fn make_qr_img(path: String, id: i32) -> PathBuf {
    let code = QrCode::new(path.as_bytes()).unwrap();
    info!("{}", path);
    let image = code.render::<Luma<u8>>().build();
    let save_path = PathBuf::from(format!("static/img/qr_code/{}.png", id));
    image.save(&save_path).unwrap();
    PathBuf::from(format!("../static/img/qr_code/{}.png", id))
}
