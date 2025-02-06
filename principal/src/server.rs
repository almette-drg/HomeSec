use std::env::VarError;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::string;
use std::time::{SystemTime, UNIX_EPOCH};

use tonic::{transport::Server, Request, Response, Status};

use image_receiver::image_receiver_server::{ImageReceiver, ImageReceiverServer};
use image_receiver::{ReceiveImageRequest, ReceiveImageReply};

pub mod image_receiver {
    tonic::include_proto!("image_receiver"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct ImageReceiverInstance {}

#[tonic::async_trait]
impl ImageReceiver for ImageReceiverInstance {
    async fn receive_image(
        &self,
        request: Request<ReceiveImageRequest>,
    ) -> Result<Response<ReceiveImageReply>, Status> {
        println!("Got a request: {:?}", request);

        let mut img_save_file_path: String = std::env::var("IMG_SAVE_FILE_PATH").expect("IMG_SAVE_FILE_PATH environment variable");
        img_save_file_path = img_save_file_path + "/" + &format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).expect("milliseconds").as_millis()) + ".png";

        let path = Path::new(&img_save_file_path);
        let file = File::create_new(&path).expect("File to be created");
        let mut writer = BufWriter::new(file);
        writer.write_all(&request.get_ref().image_data).expect("Data to be written");
        writer.flush();

        let reply = ReceiveImageReply {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = ImageReceiverInstance::default();

    //let IMAGE_SAVE_FILE_PATH: Result<String, VarError> = std::env::var("IMG_SAVE_FILE_PATH");
    std::env::var("IMG_SAVE_FILE_PATH").expect("IMG_SAVE_FILE_PATH environment value");
    Server::builder()
        .add_service(ImageReceiverServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}