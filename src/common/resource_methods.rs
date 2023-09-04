#[macro_export]
macro_rules! rpc_noparam_method {
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

#[macro_export]
macro_rules! rpc_chmod_method {
    ($rpc_method:expr) => {
        /// Changes the permissions of the resource
        pub fn chmod(&self, perms_oct: Permissions) -> Result<(), Errors> {
            let permissions_bits = PermissionsBits::from(perms_oct);

            self.chmod_raw(permissions_bits)
        }

        /// Compared to chmod this method allow to pass -1 for each bit in order to
        /// leave bit unchanged
        pub fn chmod_raw(&self, perms_bits: PermissionsBits) -> Result<(), Errors> {
            let resp_txt = self.controller.client.call(
                $rpc_method,
                vec![
                    self.id.into(),
                    (perms_bits.0 as i32).into(),
                    (perms_bits.1 as i32).into(),
                    (perms_bits.2 as i32).into(),
                    (perms_bits.3 as i32).into(),
                    (perms_bits.4 as i32).into(),
                    (perms_bits.5 as i32).into(),
                    (perms_bits.6 as i32).into(),
                    (perms_bits.7 as i32).into(),
                    (perms_bits.8 as i32).into(),
                ],
            )?;

            self.controller.parse_id_resp(resp_txt)?;

            Ok(())
        }
    };
}
