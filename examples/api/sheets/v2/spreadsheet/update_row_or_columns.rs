use std::env;

use dotenvy::dotenv;

use open_lark::{
    prelude::LarkClient, service::sheets::v2::sheet_row_col::UpdateDimensionRangeRequest,
};

/// 插入行列
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = UpdateDimensionRangeRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .sheet_id("0ae03b")
        .major_dimension("ROWS")
        .start_index(1)
        .end_index(2)
        .visible(false)
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .update_dimension_range(req, None)
        .await
        .unwrap();

    if let Some(data) = resp.data {
        println!("update-rows-or-columns spreadsheet response: {:#?}", data);
    }
}
