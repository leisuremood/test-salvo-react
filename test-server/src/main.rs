use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::{fs::OpenOptions, io::Write};

use salvo::prelude::*;
use urlencoding::decode;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct AcctInfo {
    pub vid: usize,
    #[serde(rename = "loginId")]
    pub account: String,
    #[serde(rename = "loginPwd")]
    pub password: String,
    pub name: String,
    pub phone: String,
    pub photo: String,
    #[serde(rename = "roleId")]
    pub rid: usize,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct RoleInfo {
    pub vid: usize,
    #[serde(rename = "roleId")]
    pub rid: usize,
    #[serde(rename = "roleName")]
    pub rname: String,
    #[serde(skip)]
    address: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
struct ReplyMsg0 {
    pub message: String,
    pub success: bool,
}
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
struct ReplyMsg {
    pub message: String,
    pub success: bool,
    pub token: String,
}

#[handler]
async fn hello_world(_req: &mut Request, res: &mut Response) {
    res.render("Hello World!");
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get(hello_world)
        .push(Router::with_path("/login").post(login_check))
        .push(Router::with_path("/upload").post(upload_file))
        .push(
            Router::with_path("/role")
                .push(Router::with_path("list").get(role_list))
                .push(Router::with_path("get").get(role_get))
                .push(Router::with_path("update").post(role_update))
                .push(Router::with_path("add").post(role_add))
                .push(Router::with_path("delete").get(role_del)),
        )
        .push(
            Router::with_path("/acct")
                .push(Router::with_path("list").get(acct_list))
                .push(Router::with_path("get").get(acct_get))
                .push(Router::with_path("update").post(acct_update))
                .push(Router::with_path("add").post(acct_add))
                .push(Router::with_path("delete").get(acct_del)),
        );
    let listener = TcpListener::bind("127.0.0.1:8000");
    Server::new(listener).serve(router).await;
}

#[handler]
async fn login_check(req: &mut Request, res: &mut Response) {
    let qmsg = req.parse_json::<AcctInfo>().await.unwrap();
    // println!("@login check {:#?}", qmsg);
    // let mut fid = "".to_string();
    // if let Some(id) = req.get_param::<String>("id") {
    //     println!("{}", id);
    //     fid = id;
    //     let mut file = std::fs::File::create(fid).expect("create failed");
    //     file.write_all(msg.as_bytes());
    // } else {
    //     println!("get nil");
    // }
    let reply = ReplyMsg {
        message: "welcome✨星管理系统!".to_string(),
        success: true,
        token: "xxxxsuccessyyyyyyyyy".to_string(),
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn upload_file(req: &mut Request, res: &mut Response) {
    let Some(file) = req.file("file").await else {
	println!("not find file from req");
	res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("file not found in request"));
	return;
    };
    // println!("@upload {:#?}", file);

    // let file = req.form::<String>("file").await;
    // let qmsg = req.body().unwrap();
    #[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
    struct ReplyMsg {
        pub message: String,
        pub success: bool,
        pub filename: String,
    }
    let reply = ReplyMsg {
        message: "成功上传!".to_string(),
        success: true,
        filename: "xxxxsuccessyyyyyyyyy".to_string(),
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn role_list(res: &mut Response) {
    let data = vec![
        RoleInfo {
            vid: 1,
            rid: 1,
            rname: "胡彦祖".to_string(),
            address: "西湖区湖底公园1号".to_string(),
        },
        RoleInfo {
            vid: 2,
            rid: 2,
            rname: "胡彦斌".to_string(),
            address: "西湖区湖底公园1号".to_string(),
        },
    ];
    res.render(Json(data));
    res.set_status_code(StatusCode::OK);
}
#[handler]
async fn role_get(req: &mut Request, res: &mut Response) {
    // let role_id = req.params().get("roleId").cloned().unwrap();
    let vid = req.query::<usize>("vid").unwrap();
    println!("@role get {vid}");
    let msg = RoleInfo {
        vid,
        rid: vid,
        rname: "胡彦斌".to_string(),
        address: "西湖区湖底公园1号".to_string(),
    };
    // let reply = ReplyMsg0 {
    //     message: "添加成功!".to_string(),
    //     success: true,
    // };
    res.render(Json(msg));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn role_update(req: &mut Request, res: &mut Response) {
    let role = req.parse_json::<RoleInfo>().await.unwrap();
    // println!("get {role_id}");
    let reply = ReplyMsg0 {
        message: "更改成功!".to_string(),
        success: true,
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn role_add(req: &mut Request, res: &mut Response) {
    // println!("{:#?}", req);
    let reply = ReplyMsg0 {
        message: "添加成功!".to_string(),
        success: true,
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn role_del(req: &mut Request, res: &mut Response) {
    let reply = ReplyMsg0 {
        message: "删除成功!".to_string(),
        success: true,
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn acct_list(req: &mut Request, res: &mut Response) {
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
    struct Pagination {
    #[serde(rename = "pageSize")]
	page_size: u32,
    #[serde(rename = "pageNo")]
	page_no: u32,
    }
    let param = req.parse_queries::<Pagination>();
    let param = param.unwrap_or_default();
    println!("@acct_list {:#?}", param);
    let data = vec![
        AcctInfo {
            vid: 1,
            rid: 1,
            account: "胡彦祖".to_string(),
            name: "胡彦祖".to_string(),
            phone: "13727784900".to_string(),
            ..Default::default()
        },
        AcctInfo {
            vid: 2,
            rid: 2,
            account: "胡彦斌".to_string(),
            name: "胡彦斌".to_string(),
            phone: "13727784900".to_string(),
            ..Default::default()
        },
    ];
    res.render(Json(data));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn acct_get(req: &mut Request, res: &mut Response) {
    // println!("{:#?}", req);
    // let role_id = req.params().get("roleId").cloned().unwrap();
    let vid = req.query::<usize>("vid").unwrap();
    println!("@acct get {vid}");
    let acct = AcctInfo {
        vid,
        rid: vid,
        account: "胡彦斌".to_string(),
        name: "胡彦斌".to_string(),
        phone: "13727784900".to_string(),
        ..Default::default()
    };
    res.render(Json(acct));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn acct_del(req: &mut Request, res: &mut Response) {
    let reply = ReplyMsg0 {
        message: "删除成功!".to_string(),
        success: true,
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn acct_update(req: &mut Request, res: &mut Response) {
    let acct = req.parse_json::<AcctInfo>().await.unwrap();
    // println!("get {role_id}");
    let reply = ReplyMsg0 {
        message: "更改成功!".to_string(),
        success: true,
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

#[handler]
async fn acct_add(req: &mut Request, res: &mut Response) {
    let acct = req.parse_json::<AcctInfo>().await.unwrap();
    // println!("{:#?}", req);
    let reply = ReplyMsg0 {
        message: "添加成功!".to_string(),
        success: true,
    };
    res.render(Json(reply));
    res.set_status_code(StatusCode::OK);
}

