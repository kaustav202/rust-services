use tasks::task_service_server::{TaskService, TaskServiceServer};
use tasks::{TaskRequest, TaskResponse, TaskStatusRequest, TaskStatusResponse};

use tonic::{transport::Server, Request, Response, Status};

pub mod tasks {
    tonic::include_proto!("tasks");
}

#[derive(Default)]
struct TaskHandler {}

#[tonic::async_trait]
impl TaskService for TaskHandler {
    async fn submit_task(
        &self,
        request: Request<TaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        // Process the task request
        // ...

        let response = TaskResponse {
            taskId: request.into_inner().taskId,
            status: "Task received".into(),
        };

        Ok(Response::new(response))
    }

    async fn get_task_status(
        &self,
        request: Request<TaskStatusRequest>,
    ) -> Result<Response<TaskStatusResponse>, Status> {
        // Retrieve the task status
        // ...

        let response = TaskStatusResponse {
            status: "Task in progress".into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let handler = TaskHandler::default();
    let task_service = TaskServiceServer::new(handler);

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(task_service)
        .serve(addr)
        .await?;

    Ok(())
}
