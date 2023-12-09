pub mod grpc_fs {
    tonic::include_proto!("grpc_fs");
}

pub mod grpc_data_transfer {
    tonic::include_proto!("grpc_data_transfer");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
