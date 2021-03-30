use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let credentials = Credentials::from_env_specific(
        Some("S3_ACCESS_KEY_ID"),
        Some("S3_SECRET_ACCESS_KEY"),
        None,
        None,
    ).unwrap();
    let bucket = std::env::var("S3_BUCKET").unwrap().to_string();
    let region = Region::Custom {
        region: std::env::var("S3_REGION").unwrap().into(),
        endpoint: std::env::var("S3_ENDPOINT").unwrap().into(),
    };
    let bucket = Bucket::new(&bucket, region.clone(), credentials.clone()).unwrap();
    let (_, code) = bucket.put_object("test.jpg", b"testtest").await.unwrap();
    assert_eq!(200, code);
}
