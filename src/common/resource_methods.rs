// TODO: allow to pass variable number of parameters,
// for now this only allow
#[macro_export]
macro_rules! rpc_delete_method {
    ($method_name:ident, $rpc_method:expr) => {
        pub fn $method_name(&self) -> Result<(), Errors> {
            let (success, err) = self
                .controller
                .client
                .call($rpc_method, vec![self.id.into()])?;

            if success {
                Ok(())
            } else {
                Err(Errors::OpenNebula(err))
            }
        }
    };
}

