extern crate grpcio;
extern crate protobuf;
extern crate futures;

#[path="../kontakto-common/protos/kontakto.rs"]
mod kontakto;
#[path="../kontakto-common/protos/kontakto_grpc.rs"]
mod kontakto_grpc;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use kontakto::{CreateContactRequest,
               UpdateContactRequest,
               DeleteContactRequest,
               GetAllContactsRequest};

use kontakto_grpc::KontaktoClient;


fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = KontaktoClient::new(ch);

    // Create a `CreateContactRequest`
    let req = CreateContactRequest::new();
    client.create_contact(&req).expect("rpc");

    // Create a `UpdateContactRequest`
    let req = UpdateContactRequest::new();
    client.update_contact(&req).expect("rpc");

    // Create a `DeleteContactRequest`
    let req = DeleteContactRequest::new();
    client.delete_contact(&req).expect("rpc");

    // Create a `GetAllContactsRequest`
    let req = GetAllContactsRequest::new();
    client.get_all_contacts(&req).expect("rpc");
}
