use dbbyte::{ClientOptions, DBByte, SignOptions, UploadOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DBBYTE_API_KEY")?;
    let dbb = DBByte::new(ClientOptions::new(api_key))?;

    let uploaded = dbb.objects().upload(
        "my-bucket",
        UploadOptions {
            key: "images/logo.png".to_string(),
            body: std::fs::read("logo.png")?,
            content_type: Some("image/png".to_string()),
            overwrite: false,
        },
    )?;
    println!("uploaded: {} ({} bytes)", uploaded.key, uploaded.size);

    let signed = dbb.objects().sign(
        "my-bucket",
        "images/logo.png",
        SignOptions {
            expires_in: Some(3600),
            ..Default::default()
        },
    )?;
    println!("signed URL: {}", signed.url);

    Ok(())
}
