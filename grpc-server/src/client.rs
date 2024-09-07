use tasks::task_service_client::TaskServiceClient;
use tasks::{TaskRequest, TaskStatusRequest};

use tonic::Request;

pub mod tasks {
    tonic::include_proto!("tasks");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut client = TaskServiceClient::connect(addr).await?;

    // Submit a task
    let request = tonic::Request::new(TaskRequest {
        taskId: "123".into(),
        taskPayload: "Do something".into(),
    });

    let response = client.submit_task(request).await?;
    println!("Response: {:?}", response);

    // Get task status
    let request = Request::new(TaskStatusRequest {
        taskId: "123".into(),
    });

    let response = client.get_task_status(request).await?;
    println!("Task status: {:?}", response);

    Ok(())
}
