/// RUSTでのモジュール化
/// 
/// RUSTでは、PACKAGE > CRATE > モジュール > 機能にアクセスするためのパス設定（useによる）
///
/// を持って、複数のファイルをまたいで機能をグループ化する。
/// mod でモジュールを設定しつつ、そのモジュール内では公開設定を行う。
/// pub　がPUBLICである。
/// 
/// CRATEは、実行可能なBINARY、他のCRATEで使用されることが前提とされたLIBRARY二つの状態で構築できる。
/// 
/// また、use によって機能を読み込む。
/// 機能とは、関数であり、より抽象性を持っているTraitでもある。
/// それらを読み込み、または、読み込みながら実装（Traitによる実装の強制）しながら新しい機能を構築する。
/// 
/// Traitは結局、CLASSの継承による機能の拡張に近い。
/// 
/// また、昨日の宣言は、マクロが一緒について定義されることで機能が少し変更、調整されることもある。
/// つまり、Cでのマクロ同様で、マクロは、#IFDEF などのように、独自の適用ルールを持つ。
/// マクロは、#[macro_export]、 macro_rules! などから設定される。
/// 
/// 
/// 
/// 

/// RUST USING AXUM
/// https://github.com/tokio-rs/axum/tree/main/examples/readme
/// 


/// AXUM
/// HTTPリクエストの処理、サーバLISTENINGを行うためのライブラリ
/// 
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

/// SERDE
/// JSONでの情報のやり取りを行うため、SERIALIZEとDE-SERIALIZEを行う必要があるが、そのための機能
/// 
use serde::{Deserialize, Serialize};

/// HTTP上でのIPアドレス表現のための機能
/// 
use std::net::SocketAddr;

use std::env;



/// #[tokip::main] マクロ 
/// このマクロは、基本的な設定を簡単に行うために存在する。
///
/// #[tokio::main]
/// async fn main() {
///     println!("Hello world");                 -----------------
/// }                                                            |
///                                                              |
/// ------------------------------------------                   |
/// #[tokio::main] を使用しない場合、下のようになる。                　|
///                                                              |
///                                                              |
/// fn main() {                                                  |
///     tokio::runtime::Builder::new_multi_thread()              |
///         .enable_all()                                        |
///         .build()                                             |
///         .unwrap()                                            |
///         .block_on(async {                                    |
///             println!("Hello world");                 <<<------
///         })
/// }
/// 
/// つまり、マクロは、もちろん設定次第ではあろうが、追加機能の実行を行うものである。
///                                         ~~~~~~~~~~~~~~~~~~~~
/// 
/// 
#[tokio::main]
async fn main() {

    // tracing を初期化
    // tracing はDEBUG用文字列を表示させるために使う
    tracing_subscriber::fmt::init();


    let app = create_app();


    // IPアドレスを設定
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // tracing でDEBUG文字列を表示
    tracing::debug!("listening on {}", addr);
    
    // サーバの起動
    // AXUMパッケージのServerモジュールは、Hyperの機能を流用しているようだ。
    // 
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();



}




fn create_app() -> Router {

    // ROUTERの設定
    Router::new()
        // GET リクエストで、パスが　/の場合、こちらを。 
        .route("/", get(root))
        // POST リクエストで、パスが/users　なら、create_user関数を実行。
        .route("/users", post(create_user))

}



// -----------------------------------------------
// CONTROLLER 部分
//
// 本サンプルでは、ただの文字をレスポンスしているだけなので、
// 特にVIEWに当たる部分はない。


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}




async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {


    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };



    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))



}


// -----------------------------------------------
// MODEL 部分


// the input to our `create_user` handler
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}






// --------------------------------------------------------------------------
// TEST 部分



#[cfg(test)]
mod test {

    // super は、相対的に、ここからみて上の階層を指す。
    // つまり、このtestモジュールからすれば、このmain.rsファイルを指す。
    // よって、このファイルの中にあるStructなどの要素を読み込むことができる。
    use super::*;

    // このテストのために必要になる　AXUMからの機能
    // AXUMライブラリの機能なので、リクエスト関連のHEADERなどの機能を取り入れている。
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };

    // サーバから帰ってきたレスポンスを確認するため、
    // まずデータをバイト型に変換する必要がある。
    // そのための機能。
    use tower::ServiceExt;

    // テスト１
    // "/" パスにリクエストを送って、"Hello, World!"を送ってくれるのかをテストする。
    //
    #[tokio::test]
    async fn should_return_string() {

        // リクエストを設定
        // "/"　のパスなので、rootメソッドに設定されているように、"Hello, World!"を
        // 返してくれるはず。まだここでは送信してはいない。
        let req = Request::builder()
                                    .uri("/")
                                    .body(Body::empty())
                                    .unwrap();

        // レスポンスを生成。
        //　ROUTERを生成し、towerの機能であるoneshot関数を使用し、
        // 上記で設定したリクエストを送信する。
        // この処理は、瞬時に行われることはないし、そもそも非同期な処理で定義されている。
        // よって、処理の終了をawaitによって待つ必要がある。
        // また、エラー処理に関してはunwrap() をすることで、まずはResult型のデータになることを
        // 簡略化して処理している。
        let res = create_app()
                            .oneshot(req)
                            .await.unwrap();

        // それでも、このresはそのままでは中を除くことができない。
        //
        // まずは、Bytesに変換し、それをさらに文字列に変換する必要がある。
        // よって、hyper のリクエストBodyを処理する機能を使用する。
        // 上記のように、エラー関連の処理は簡略化している。
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        // ここまできて、ようやく文字列としてみれるようになった
        // レスポンスのBodyと、本来送られるべき値である"Hello, World!"を
        // assert_eq! で比較しテストを行う。
        assert_eq!( body, "Hello, World!" );

    }




    // テスト２
    // "/users" パスにPOSTリクエストを送って、送ったusernameと同じデータを送ってくれるのかをテストする。
    #[tokio::test]
    async fn should_return_samedata_from_post() {

        // テスト１と同じだが、ここではPOSTリクエストを送っている。
        // JSON形式のBODYを送っている。
        let req = Request::builder()
                    .uri("/users")
                    .method(Method::POST)
                    .header( header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(r#"{
                        "username": "CHOI SIYOUNG"
                    }"#))
                    .unwrap();

        // リクエストを送信し、レスポンスを受け取る。
        let res = create_app()
                    .oneshot(req)
                    .await
                    .unwrap();

        // towerの機能を使って、バイトに変換。
        let bytes = hyper::body::to_bytes(res.into_body())
                        .await
                        .unwrap();

        // 文字列に変換。
        let body: String = String::from_utf8(bytes.to_vec())
                            .unwrap();

        // PLAINなテキスト状態のJSONデータを、
        // User型のデータに変換する。
        let user: User = serde_json::from_str(&body)
                            .expect("COULD NOT CONVERT!");

        // 最後に同じかをテスト。
        assert_eq!(

            user,
            User {
                id:1337,
                username: "CHOI SIYOUNG".to_string(),
            }

        );

    }





}

