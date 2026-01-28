pub mod v1 {
    pub mod agent {
        tonic::include_proto!("v1.agent");
    }

    pub mod query {
        tonic::include_proto!("v1.query");
    }

    pub mod schema {
        tonic::include_proto!("v1.schema");
    }
}
