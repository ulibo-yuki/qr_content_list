use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Local;
use log::info;
use serde::Deserialize;
use std::path::PathBuf;

//style.css path is "<link rel=\"stylesheet\" href=\"../static/css/style.css\">"

mod data;

#[get("/")]
async fn home() -> impl Responder {
    let mut body_str = "".to_string();
    body_str += include_str!("../static/home_header.html");
    // body_str += "<h1>Home</h1>";
    body_str += "<div class=\"home\">
                    <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">
                    <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>
                    <link href=\"https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Kiwi+Maru:wght@300;400;500&family=Noto+Sans+JP:wght@100..900&family=RocknRoll+One&display=swap\" rel=\"stylesheet\">
                    <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">
                    <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>
                    <link href=\"https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Kiwi+Maru:wght@300;400;500&family=Noto+Sans+JP:wght@100..900&family=RocknRoll+One&display=swap\" rel=\"stylesheet\">
                    <link rel=\"stylesheet\" href=\"../static/css/home.css\">
                    <link rel=\"stylesheet\" href=\"../static/css/style.css\">
                    <div class=\"links\">
                        <a href=\"qr_list\"><div class=\"link\">
                            <p class=\"link_title\">二次元ボックス</p>
                            <p>QRコードで収納の中身の登録・確認ができる</p>
                        </div></a>
                    </div>
                </div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok().body(body_str)
}

#[get("/qr_list")]
pub async fn qr_list() -> impl Responder {
    let qrs_list = data::get_all();
    let mut body_str = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += "<h1>QR 一覧</h1>
                <link rel=\"stylesheet\" href=\"../static/css/style.css\">
                <link rel=\"stylesheet\" href=\"../static/css/qr_list.css\">
                <a href=\"/qr_list/new\" class=\"new_btn\">作成</a>
                <div class=\"qr_content_list\">";
    for item in &qrs_list {
        body_str += &format!("<div class=\"qr_content\"><a href=\"qr_list/{id}\">
                            <p class=\"content_title\">{title}</p>
                            <div class=\"content_info\">
                            <nav class=\"content_list\">
                            <ul>", id=item.id, title=item.title);
        for list in &item.content_list {
            body_str += &format!("<li>{}</li>", list);
        }
        body_str += &format!("</ul></nav><img src=\"{img_path}\" alt=\"qr image path\"></div><p class=\"edit_time\">編集日: {edit_time}</p></a></div>", img_path=item.qr_img_path.to_string_lossy().into_owned(), edit_time=item.last_edit_time);
    };
    body_str += "</div>";
    body_str += include_str!("../static/footer.html");

    info!("called index");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/qr_list/{id}")]
pub async fn show(info: web::Path<i32>) -> impl Responder {
    info!("called show");
    let info = info.into_inner();
    let post = data::get(info);
    let mut body_str = "".to_string();
    body_str += include_str!("../static/header.html");
    // 個別表示
    if post.id != 0 {
        body_str += &format!(
        "<div class=\"pare\">
            <p class=\"content_title\">{}</p>
            <div class=\"content\">
                <link rel=\"stylesheet\" href=\"../static/css/show.css\">
                <div class=\"content_list\">
                    <ul>", post.title);
        for item in post.content_list {
            body_str += &format!("<li>{}</li>", item);
        }
        body_str += &format!("</ul></div><img src=\"{}\" alt=\"qr png\"></div><p class=\"datetime\">last edited by: {}", post.qr_img_path.to_string_lossy().into_owned().trim(), post.last_edit_time);
        body_str += &format!("<div class=\"edit_remove_btns\"><a href=\"/qr_list/{}/edit\" class=\"three_btns\">編集</a>", info);
        body_str += &format!("<a href=\"/qr_list/{}/delete\" class=\"three_btns\">削除</a>", info);
    } else {
        body_str += "見つかりません。";
    }
    body_str += "<a href=\"/qr_list\" class=\"three_btns\">一覧へ</a></div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok()
        .content_type("text.html; charset=utf-8")
        .body(body_str)
}

#[get("/qr_list/new")]
pub async fn new() -> impl Responder {
    let mut body_str = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += include_str!("../static/form.html");
    body_str += include_str!("../static/footer.html");

    body_str = body_str.replace("{{action}}", "create");
    body_str = body_str.replace("{{id}}", "0");
    body_str = body_str.replace("{last_edit_time}", "");
    body_str = body_str.replace("{{title}}", "");
    body_str = body_str.replace("{{content}}", "");
    body_str = body_str.replace("{{button}}", "登録");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[derive(Deserialize, Debug)]
pub struct CreateFrom {
    // id: i32,
    // last_edit_time: String,
    title: String,
    content: String,
}

#[post("/qr_list/create")]
pub async fn create(params: web::Form<CreateFrom>) -> impl Responder {
    let now = Local::now();
    let mut content_list = data::TwoDBoxList {
        id: 0,
        last_edit_time: now.format("%y-%m-%d %H:%M").to_string(),
        title: params.title.clone(),
        content_list: string_to_vec(params.content.clone()),
        qr_img_path: PathBuf::new(),
    };
    content_list = data::create(content_list);
    web::Redirect::to(format!("/qr_list/{}", content_list.id)).see_other()
}

fn string_to_vec(str: String) -> Vec<String> {
    str.trim_end_matches(',').split(',').map(|s| s.to_string()).collect()
}

pub async fn not_found() -> impl Responder {
    let mut body_str = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += include_str!("../static/not_found.html");
    body_str += include_str!("../static/footer.html");
    HttpResponse::NotFound().body(body_str)
}

#[get("/qr_list/{id}/delete")]
pub async fn destroy(info: web::Path<i32>) -> impl Responder {
    info!("called destroy");
    let info = info.into_inner();
    data::remove(info);
    web::Redirect::to("/qr_list").see_other()
}
