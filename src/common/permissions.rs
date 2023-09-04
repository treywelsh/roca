use std::fmt::{Debug, Display};

// Rights bits:
// U for use
// M for manage
// A for admin
pub mod flags {
    pub const USR_UMA: u16 = 0o000700;
    pub const USR_U: u16 = 0o000400;
    pub const USR_M: u16 = 0o000200;
    pub const USR_A: u16 = 0o000100;
    pub const GRP_UMA: u16 = 0o000070;
    pub const GRP_U: u16 = 0o000030;
    pub const GRP_M: u16 = 0o000020;
    pub const GRP_A: u16 = 0o000010;
    pub const OTH_UMA: u16 = 0o000007;
    pub const OTH_U: u16 = 0o000004;
    pub const OTH_M: u16 = 0o000002;
    pub const OTH_A: u16 = 0o000001;
}

pub struct PermissionsBits(
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);

pub struct Permissions(pub u16);

// TODO: check 0/1 bitfield value
impl From<PermissionsBits> for Permissions {
    fn from(bits: PermissionsBits) -> Self {
        Permissions(
            ((bits.0 as u16) << 8)
                + ((bits.1 as u16) << 7)
                + ((bits.2 as u16) << 6)
                + ((bits.3 as u16) << 5)
                + ((bits.4 as u16) << 4)
                + ((bits.5 as u16) << 3)
                + ((bits.6 as u16) << 2)
                + ((bits.7 as u16) << 1)
                + (bits.8 as u16),
        )
    }
}

impl Debug for PermissionsBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PermissionsBits")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .field(&self.3)
            .field(&self.4)
            .field(&self.5)
            .field(&self.6)
            .field(&self.7)
            .field(&self.8)
            .finish()
    }
}

impl From<Permissions> for PermissionsBits {
    fn from(perms_oct: Permissions) -> Self {
        PermissionsBits(
            (perms_oct.0 & 0o000001) as u8,
            (perms_oct.0 & 0o000002) as u8,
            (perms_oct.0 & 0o000004) as u8,
            ((perms_oct.0 & 0o000010) >> 3) as u8,
            ((perms_oct.0 & 0o000020) >> 3) as u8,
            ((perms_oct.0 & 0o000040) >> 3) as u8,
            ((perms_oct.0 & 0o000100) >> 6) as u8,
            ((perms_oct.0 & 0o000200) >> 6) as u8,
            ((perms_oct.0 & 0o000400) >> 6) as u8,
        )
    }
}

const PERMISSION_STR: [&str; 8] = ["---", "--a", "-m-", "-ma", "u--", "u-a", "um-", "uma"];

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(PERMISSION_STR[((self.0 & 0o700) >> 6) as usize])?;
        f.write_str(PERMISSION_STR[((self.0 & 0o070) >> 3) as usize])?;
        f.write_str(PERMISSION_STR[(self.0 & 0o007) as usize])?;

        Ok(())
    }
}

#[cfg(test)]
mod helpers {
    use crate::common::permissions::{flags, Permissions};

    #[test]
    fn check_str_other_use() {
        assert_eq!(Permissions(flags::OTH_U).to_string(), "------u--");
    }

    #[test]
    fn check_str_all_manage() {
        assert_eq!(
            Permissions(flags::USR_M | flags::GRP_M | flags::OTH_M).to_string(),
            "-m--m--m-"
        );
    }

    #[test]
    fn check_str_complex() {
        assert_eq!(
            Permissions(flags::USR_U | flags::GRP_UMA | flags::OTH_A).to_string(),
            "u--uma--a"
        );
    }
}
