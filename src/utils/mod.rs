extern crate rusoto_core;
extern crate rusoto_ecr;

use std::env;

use rusoto_core::Region;
use rusoto_ecr::{BatchGetImageRequest, Ecr, EcrClient, ImageIdentifier};

pub mod writer;


pub async fn get_tag(repo: &str, tag: &str) -> Result<String, String> {
    let client = EcrClient::new(Region::UsEast1);

    let registry_id = if env::var("ECR_REGISTRY_ID").is_ok() {
        Some(env::var("ECR_REGISTRY_ID").unwrap())
    } else {
        None
    };
    let repository_name = format!("{}{}", env::var("ECR_IMAGE_PREFIX").unwrap_or("".into()), repo);

    let get_img_input = BatchGetImageRequest {
    accepted_media_types: None,
    image_ids: vec![ImageIdentifier { image_tag: Some(tag.into()), ..Default::default() }],
    registry_id: registry_id,
    repository_name: repository_name,
    };

    let output: Result<String, String> = match client.batch_get_image(get_img_input).await {
        Ok(output) => match output.images {
            Some(images) => {
                let mut image_output = Err("No image tags found.".to_string());
                if images.len() > 1 {
                    image_output = Err("Multiple tags found?".into());
                } else {
                    for img in images {
                        let image_tag = img.image_id.clone().unwrap().image_tag.unwrap();
                        let image_sha = img.image_id.unwrap().image_digest.unwrap();
                        image_output = Ok(format!(
                            "Image Tag: {}\nImage Sha: {}",
                            image_tag,
                            image_sha
                        ));
                    }
                }
                image_output
            },
            None => return Err("No image tags".into()),
        },
        Err(error) => return Err(error.to_string()),
  };

  output

}
