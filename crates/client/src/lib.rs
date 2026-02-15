use common::hello::{HelloRequest, greeter_client::GreeterClient};
use tonic::transport::{Channel, Error as TransportError};

pub struct Client {
    inner: GreeterClient<Channel>,
}

impl Client {
    pub async fn connect(server_url: impl Into<String>) -> Result<Self, TransportError> {
        let inner = GreeterClient::connect(server_url.into()).await?;
        Ok(Self { inner })
    }

    pub async fn say_hello(&mut self, name: impl Into<String>) -> Result<String, tonic::Status> {
        let request = tonic::Request::new(HelloRequest { name: name.into() });
        let response = self.inner.say_hello(request).await?;
        Ok(response.into_inner().message)
    }
}
