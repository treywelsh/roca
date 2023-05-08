#[macro_export]
macro_rules! rpc_delete_method {
    ($method_name:ident, $rpc_method:expr) => {
        pub fn $method_name(&self) -> Result<(), Errors> {
            let resp_txt = self
                .controller
                .client
                .call($rpc_method, vec![self.id.into()])?;

            self.controller.parse_id_resp(resp_txt)?;

            Ok(())
        }
    };
}
