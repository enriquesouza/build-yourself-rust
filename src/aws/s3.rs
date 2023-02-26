use std::collections::HashMap;

// I need to list all files form aws s3 bucket.
// I am using aws-sdk-rust. I am able to list all files from bucket but I am not able to get the file name. I am using the following code to list all files from bucket.
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

// Lists the objects in a bucket.
// snippet-start:[s3.rust.list-objects]
pub async fn show_s3_objects(
    bucket: &str,
) -> Result<HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let resp = client.list_objects_v2().bucket(bucket).send().await?;

    let folders = resp.common_prefixes.unwrap_or_default();
    let files = resp.contents.unwrap_or_default();

    let mut items = Vec::new();
    items.extend(folders.into_iter().map(|p| p.prefix.unwrap()));
    items.extend(files.into_iter().map(|c| c.key.unwrap()));

    let mut result = std::collections::HashMap::new();
    let mut images = std::collections::HashMap::new();

    for item in &items {
        let parts: Vec<_> = item.split('/').collect();

        if parts.len() > 1 {
            let folder_name = parts[0];
            let image_name = parts[1];

            let folder = images
                .entry(folder_name.to_owned())
                .or_insert_with(|| std::collections::HashMap::new());

            let images = folder
                .entry("images".to_owned())
                .or_insert_with(|| Vec::new());

            images.push(image_name.to_owned());
        }
    }

    result.insert("images".to_owned(), images);

    Ok(result)
}
