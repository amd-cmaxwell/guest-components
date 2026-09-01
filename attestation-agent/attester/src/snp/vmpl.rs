// Copyright (c) 2026 Advanced Micro Devices
//
// SPDX-License-Identifier: Apache-2.0
//

/// VMPrivilegeLevel represents a distinct Virtual Machine Privilege Level on an SNP-enabled confidential virtual machine (CVM)
///
/// Virtual Machine Privilege Levels (VMPLs) are hardware-enforced isolation rings (ranging from VMPL0 to VMPL3) inside an AMD Secure Encrypted Virtualization-Secure Nested Paging (AMD SEV) confidential virtual machine.
///
/// The default VMPrivilegeLevel for both this structure and for generating a TSM report on an SNP-enabled CVM is 0 (i.e. the highest privilege level).
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)] // Tells Rust to store this enum as an unsigned 8-bit integer
pub enum VMPrivilegeLevel {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl VMPrivilegeLevel {
    /// MOST_PRIVILEGED returns the lowest VMPL ring (the highest privilege level)
    pub const MOST_PRIVILEGED: Self = Self::Zero;
    /// LEAST_PRIVILEGED returns the highest VMPL ring (the lowest privilege level)
    pub const LEAST_PRIVILEGED: Self = Self::Three;

    /// MIN returns the lowest VMPL ring (the highest privilege level)
    pub const INT_MIN: Self = Self::MOST_PRIVILEGED;
    /// MAX returns the highest VMPL ring (the least privilege level)
    pub const INT_MAX: Self = Self::LEAST_PRIVILEGED;
}

impl TryFrom<u8> for VMPrivilegeLevel {
    type Error = std::io::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => std::result::Result::Ok(Self::Zero),
            1 => std::result::Result::Ok(Self::One),
            2 => std::result::Result::Ok(Self::Two),
            3 => std::result::Result::Ok(Self::Three),
            other => std::result::Result::Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid TSM Privilege Level: {}", other),
            )),
        }
    }
}

impl std::fmt::Display for VMPrivilegeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8) // Prints the u8 value, or map to text names
    }
}
