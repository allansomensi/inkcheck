use clap::ValueEnum;
use snmp2::v3;
use std::fmt::Display;

/// Specifies the encryption algorithms supported for SNMPv3 privacy.
///
/// Determines how the payload is encrypted when `AuthPriv` security level is used.
#[derive(Copy, Clone, ValueEnum, Debug, Default)]
pub enum AuthCipher {
    Des,
    #[default]
    Aes128,
    Aes192,
    Aes256,
}

impl Display for AuthCipher {
    /// Formats the cipher name using its CLI-compatible string representation (e.g., "aes128").
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("variant not skipped")
            .get_name()
            .fmt(f)
    }
}

impl From<AuthCipher> for v3::Cipher {
    /// Maps the CLI [`AuthCipher`] enum to the underlying library's [`v3::Cipher`] type.
    fn from(proto: AuthCipher) -> Self {
        match proto {
            AuthCipher::Des => v3::Cipher::Des,
            AuthCipher::Aes128 => v3::Cipher::Aes128,
            AuthCipher::Aes192 => v3::Cipher::Aes192,
            AuthCipher::Aes256 => v3::Cipher::Aes256,
        }
    }
}
