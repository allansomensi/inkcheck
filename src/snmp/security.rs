use clap::ValueEnum;
use snmp2::v3;
use std::fmt::Display;

/// Specifies the encryption algorithms supported for SNMPv3 privacy.
///
/// Determines how the payload is encrypted when [`SecurityLevel::AuthPriv`] is used.
#[derive(Copy, Clone, ValueEnum, Debug, Default)]
pub enum PrivacyProtocol {
    Des,
    #[default]
    Aes128,
    Aes192,
    Aes256,
}

impl Display for PrivacyProtocol {
    /// Formats the cipher name using its CLI-compatible string representation (e.g., "aes128").
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("variant not skipped")
            .get_name()
            .fmt(f)
    }
}

impl From<PrivacyProtocol> for v3::Cipher {
    /// Maps the CLI [`PrivacyProtocol`] enum to the underlying library's [`v3::Cipher`] type.
    fn from(proto: PrivacyProtocol) -> Self {
        match proto {
            PrivacyProtocol::Des => v3::Cipher::Des,
            PrivacyProtocol::Aes128 => v3::Cipher::Aes128,
            PrivacyProtocol::Aes192 => v3::Cipher::Aes192,
            PrivacyProtocol::Aes256 => v3::Cipher::Aes256,
        }
    }
}

/// Specifies the hashing algorithms supported for SNMPv3 authentication.
///
/// Determines how the message digest is calculated to verify sender identity and data integrity.
#[derive(Copy, Clone, ValueEnum, Debug, Default)]
pub enum AuthProtocol {
    Md5,
    #[default]
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl Display for AuthProtocol {
    /// Formats the protocol name using its CLI-compatible string representation (e.g., "sha1").
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("variant not skipped")
            .get_name()
            .fmt(f)
    }
}

impl From<AuthProtocol> for v3::AuthProtocol {
    /// Maps the CLI [`AuthProtocol`] enum to the underlying library's [`v3::AuthProtocol`] type.
    fn from(proto: AuthProtocol) -> Self {
        match proto {
            AuthProtocol::Md5 => v3::AuthProtocol::Md5,
            AuthProtocol::Sha1 => v3::AuthProtocol::Sha1,
            AuthProtocol::Sha224 => v3::AuthProtocol::Sha224,
            AuthProtocol::Sha256 => v3::AuthProtocol::Sha256,
            AuthProtocol::Sha384 => v3::AuthProtocol::Sha384,
            AuthProtocol::Sha512 => v3::AuthProtocol::Sha512,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, ValueEnum, Debug, Default)]
pub enum SecurityLevel {
    NoAuthNoPriv,
    #[default]
    AuthNoPriv,
    AuthPriv,
}

impl Display for SecurityLevel {
    /// Formats the protocol name using its CLI-compatible string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("variant not skipped")
            .get_name()
            .fmt(f)
    }
}
