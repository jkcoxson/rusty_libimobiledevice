// jkcoxson
// Enums for libimobiledevice's error codes

#[derive(PartialEq, Debug)]
pub enum LockdowndError {
    Success,
    InvalidArg,
    InvalidConf,
    PlistError,
    PairingFailed,
    SslError,
    DictError,
    RecieveTimeout,
    MuxError,
    NoRunningSession,
    InvalidResponse,
    MissingKey,
    MissingValue,
    GetProhibited,
    SetProhibited,
    RemoveProhibited,
    ImmutableValue,
    PasswordProtected,
    UserDeniedPairing,
    PairingDialogueRepsonsePending,
    MissingHostId,
    InvalidHostId,
    SessionActive,
    SessionInactive,
    MissingSessionId,
    InvalidSessionId,
    MissingService,
    InvalidService,
    ServiceLimit,
    MissingPairRecord,
    SavePairRecordFailed,
    InvalidPairRecord,
    InvalidActivationRecord,
    MissingActivationRecord,
    ServiceProhibited,
    EscrowLocked,
    PairingProhibitedOverThisConnection,
    FmipProtected,
    McProtected,
    McChallengeRequired,
    UnknownError,
}

impl From<i32> for LockdowndError {
    fn from(i: i32) -> LockdowndError {
        match i {
            0 => LockdowndError::Success,
            -1 => LockdowndError::InvalidArg,
            -2 => LockdowndError::InvalidConf,
            -3 => LockdowndError::PlistError,
            -4 => LockdowndError::PairingFailed,
            -5 => LockdowndError::SslError,
            -6 => LockdowndError::DictError,
            -7 => LockdowndError::RecieveTimeout,
            -8 => LockdowndError::MuxError,
            -9 => LockdowndError::NoRunningSession,
            -10 => LockdowndError::InvalidResponse,
            -11 => LockdowndError::MissingKey,
            -12 => LockdowndError::MissingValue,
            -13 => LockdowndError::GetProhibited,
            -14 => LockdowndError::SetProhibited,
            -15 => LockdowndError::RemoveProhibited,
            -16 => LockdowndError::ImmutableValue,
            -17 => LockdowndError::PasswordProtected,
            -18 => LockdowndError::UserDeniedPairing,
            -19 => LockdowndError::PairingDialogueRepsonsePending,
            -20 => LockdowndError::MissingHostId,
            -21 => LockdowndError::InvalidHostId,
            -22 => LockdowndError::SessionActive,
            -23 => LockdowndError::SessionInactive,
            -24 => LockdowndError::MissingSessionId,
            -25 => LockdowndError::InvalidSessionId,
            -26 => LockdowndError::MissingService,
            -27 => LockdowndError::InvalidService,
            -28 => LockdowndError::ServiceLimit,
            -29 => LockdowndError::MissingPairRecord,
            -30 => LockdowndError::SavePairRecordFailed,
            -31 => LockdowndError::InvalidPairRecord,
            -32 => LockdowndError::InvalidActivationRecord,
            -33 => LockdowndError::MissingActivationRecord,
            -34 => LockdowndError::ServiceProhibited,
            -35 => LockdowndError::EscrowLocked,
            -36 => LockdowndError::PairingProhibitedOverThisConnection,
            -37 => LockdowndError::FmipProtected,
            -38 => LockdowndError::McProtected,
            -39 => LockdowndError::McChallengeRequired,
            _ => LockdowndError::UnknownError,
        }
    }
}

impl From<LockdowndError> for String {
    fn from(e: LockdowndError) -> String {
        match e {
            LockdowndError::Success => "Success".to_string(),
            LockdowndError::InvalidArg => "InvalidArg".to_string(),
            LockdowndError::InvalidConf => "InvalidConf".to_string(),
            LockdowndError::PlistError => "PlistError".to_string(),
            LockdowndError::PairingFailed => "PairingFailed".to_string(),
            LockdowndError::SslError => "SslError".to_string(),
            LockdowndError::DictError => "DictError".to_string(),
            LockdowndError::RecieveTimeout => "RecieveTimeout".to_string(),
            LockdowndError::MuxError => "MuxError".to_string(),
            LockdowndError::NoRunningSession => "NoRunningSession".to_string(),
            LockdowndError::InvalidResponse => "InvalidResponse".to_string(),
            LockdowndError::MissingKey => "MissingKey".to_string(),
            LockdowndError::MissingValue => "MissingValue".to_string(),
            LockdowndError::GetProhibited => "GetProhibited".to_string(),
            LockdowndError::SetProhibited => "SetProhibited".to_string(),
            LockdowndError::RemoveProhibited => "RemoveProhibited".to_string(),
            LockdowndError::ImmutableValue => "ImmutableValue".to_string(),
            LockdowndError::PasswordProtected => "PasswordProtected".to_string(),
            LockdowndError::UserDeniedPairing => "UserDeniedPairing".to_string(),
            LockdowndError::PairingDialogueRepsonsePending => {
                "PairingDialogueRepsonsePending".to_string()
            }
            LockdowndError::MissingHostId => "MissingHostId".to_string(),
            LockdowndError::InvalidHostId => "InvalidHostId".to_string(),
            LockdowndError::SessionActive => "SessionActive".to_string(),
            LockdowndError::SessionInactive => "SessionInactive".to_string(),
            LockdowndError::MissingSessionId => "MissingSessionId".to_string(),
            LockdowndError::InvalidSessionId => "InvalidSessionId".to_string(),
            LockdowndError::MissingService => "MissingService".to_string(),
            LockdowndError::InvalidService => "InvalidService".to_string(),
            LockdowndError::ServiceLimit => "ServiceLimit".to_string(),
            LockdowndError::MissingPairRecord => "MissingPairRecord".to_string(),
            LockdowndError::SavePairRecordFailed => "SavePairRecordFailed".to_string(),
            LockdowndError::InvalidPairRecord => "InvalidPairRecord".to_string(),
            LockdowndError::InvalidActivationRecord => "InvalidActivationRecord".to_string(),
            LockdowndError::MissingActivationRecord => "MissingActivationRecord".to_string(),
            LockdowndError::ServiceProhibited => "ServiceProhibited".to_string(),
            LockdowndError::EscrowLocked => "EscrowLocked".to_string(),
            LockdowndError::PairingProhibitedOverThisConnection => {
                "PairingProhibitedOverThisConnection".to_string()
            }
            LockdowndError::FmipProtected => "FmipProtected".to_string(),
            LockdowndError::McProtected => "McProtected".to_string(),
            LockdowndError::McChallengeRequired => "McChallengeRequired".to_string(),
            LockdowndError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum IdeviceError {
    Success,
    InvalidArg,
    UnknownError,
    NoDevice,
    NotEnoughData,
    ConnRefused,
    SslError,
    Timeout,
}

impl From<i32> for IdeviceError {
    fn from(e: i32) -> IdeviceError {
        match e {
            0 => IdeviceError::Success,
            -1 => IdeviceError::InvalidArg,
            -3 => IdeviceError::NoDevice,
            -4 => IdeviceError::NotEnoughData,
            -5 => IdeviceError::ConnRefused,
            -6 => IdeviceError::SslError,
            -7 => IdeviceError::Timeout,
            _ => IdeviceError::UnknownError,
        }
    }
}

impl From<IdeviceError> for String {
    fn from(e: IdeviceError) -> String {
        match e {
            IdeviceError::Success => "Success".to_string(),
            IdeviceError::InvalidArg => "InvalidArg".to_string(),
            IdeviceError::UnknownError => "UnknownError".to_string(),
            IdeviceError::NoDevice => "NoDevice".to_string(),
            IdeviceError::NotEnoughData => "NotEnoughData".to_string(),
            IdeviceError::ConnRefused => "ConnRefused".to_string(),
            IdeviceError::SslError => "SslError".to_string(),
            IdeviceError::Timeout => "Timeout".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum UserPrefError {
    Success,
    InvalidArg,
    NoEnt,
    InvalidConf,
    SslError,
    ReadError,
    WriteError,
    UnknownError,
}

impl From<i32> for UserPrefError {
    fn from(e: i32) -> UserPrefError {
        match e {
            0 => UserPrefError::Success,
            -1 => UserPrefError::InvalidArg,
            -2 => UserPrefError::NoEnt,
            -3 => UserPrefError::InvalidConf,
            -4 => UserPrefError::SslError,
            -5 => UserPrefError::ReadError,
            -6 => UserPrefError::WriteError,
            _ => UserPrefError::UnknownError,
        }
    }
}

impl From<UserPrefError> for String {
    fn from(e: UserPrefError) -> String {
        match e {
            UserPrefError::Success => "Success".to_string(),
            UserPrefError::InvalidArg => "InvalidArg".to_string(),
            UserPrefError::NoEnt => "NoEnt".to_string(),
            UserPrefError::InvalidConf => "InvalidConf".to_string(),
            UserPrefError::SslError => "SslError".to_string(),
            UserPrefError::ReadError => "ReadError".to_string(),
            UserPrefError::WriteError => "WriteError".to_string(),
            UserPrefError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PropertyListServiceError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    RecieveTimeout,
    NotEnoughData,
    UnknownError,
}

impl From<i32> for PropertyListServiceError {
    fn from(e: i32) -> PropertyListServiceError {
        match e {
            0 => PropertyListServiceError::Success,
            -1 => PropertyListServiceError::InvalidArg,
            -2 => PropertyListServiceError::PlistError,
            -3 => PropertyListServiceError::MuxError,
            -4 => PropertyListServiceError::SslError,
            -5 => PropertyListServiceError::RecieveTimeout,
            -6 => PropertyListServiceError::NotEnoughData,
            _ => PropertyListServiceError::UnknownError,
        }
    }
}

impl From<PropertyListServiceError> for String {
    fn from(e: PropertyListServiceError) -> String {
        match e {
            PropertyListServiceError::Success => "Success".to_string(),
            PropertyListServiceError::InvalidArg => "InvalidArg".to_string(),
            PropertyListServiceError::PlistError => "PlistError".to_string(),
            PropertyListServiceError::MuxError => "MuxError".to_string(),
            PropertyListServiceError::SslError => "SslError".to_string(),
            PropertyListServiceError::RecieveTimeout => "RecieveTimeout".to_string(),
            PropertyListServiceError::NotEnoughData => "NotEnoughData".to_string(),
            PropertyListServiceError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ServiceError {
    Success,
    InvalidArg,
    MuxError,
    SslError,
    StartServiceError,
    NotEnoughData,
    Timeout,
    UnknownError,
}

impl From<i32> for ServiceError {
    fn from(e: i32) -> ServiceError {
        match e {
            0 => ServiceError::Success,
            -1 => ServiceError::InvalidArg,
            -2 => ServiceError::MuxError,
            -3 => ServiceError::SslError,
            -4 => ServiceError::StartServiceError,
            -5 => ServiceError::NotEnoughData,
            -6 => ServiceError::Timeout,
            _ => ServiceError::UnknownError,
        }
    }
}

impl From<ServiceError> for String {
    fn from(e: ServiceError) -> String {
        match e {
            ServiceError::Success => "Success".to_string(),
            ServiceError::InvalidArg => "InvalidArg".to_string(),
            ServiceError::MuxError => "MuxError".to_string(),
            ServiceError::SslError => "SslError".to_string(),
            ServiceError::StartServiceError => "StartServiceError".to_string(),
            ServiceError::NotEnoughData => "NotEnoughData".to_string(),
            ServiceError::Timeout => "Timeout".to_string(),
            ServiceError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum InstProxyError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    OpInProgress,
    OpFailed,
    RecieveTimeout,
    AlreadyArchived,
    ApiInternalerror,
    ApplicationAlreadyInstalled,
    ApplicationMoveFailed,
    ApplicationSinfCaptureFailed,
    ApplicationSandboxFailed,
    ApplicationVerificationFailed,
    ArchiveDestructionFailed,
    BundleVerificationFailed,
    CarrierBundleCopyFailed,
    CarrierBundleDirectoryCreationFailed,
    CarrierBundleMissingSupportedSims,
    CommCenterNotificationFailed,
    ContainerCreationFailed,
    ContainerP0wnFailed,
    ContainerRemovalFailed,
    EmbeddedProfileInstallFailed,
    ExecutableTwiddleFailed,
    ExistenceCheckFailed,
    InstallMapUpdateFailed,
    ManifestCaptureFailed,
    MapGenerationFailed,
    MissingBundleExecutable,
    MissingBundleIdentifier,
    MissingBundlePath,
    MissingContainer,
    NotificationFailed,
    PackageExtractionFailed,
    PackageInspectionFailed,
    PackageMoveFailed,
    PathConversionFailed,
    RestoreConversionFailed,
    SeatbeltProfileRemovalFailed,
    StageCreationFailed,
    SymlinkFailed,
    UnknownCommand,
    ItunesArtworkCaptureFailed,
    ItunesMetadataCaptureFailed,
    DeviceOsVersionTooLow,
    DeviceFamilyNotSupported,
    PackagePatchFailed,
    IncorrectArchitecture,
    PluginCopyFailed,
    BreadcrumbFailed,
    BreadcrumbUnlockFailed,
    GeoJsonCaptureFailed,
    NewsstandArtworkCaptureFailed,
    MissingCommand,
    NotEntitled,
    MissingPackagePath,
    MissingContainerPath,
    MissingApplicationIdentifier,
    MissingAttributeValue,
    LookupFailed,
    DictCreationFailed,
    InstallProhibited,
    UninstallProhibited,
    MissingBundleVersion,
    UnknownError,
}

impl From<i32> for InstProxyError {
    fn from(e: i32) -> InstProxyError {
        match e {
            0 => InstProxyError::Success,
            -1 => InstProxyError::InvalidArg,
            -2 => InstProxyError::PlistError,
            -3 => InstProxyError::ConnFailed,
            -4 => InstProxyError::OpInProgress,
            -5 => InstProxyError::OpFailed,
            -6 => InstProxyError::RecieveTimeout,
            -7 => InstProxyError::AlreadyArchived,
            -8 => InstProxyError::ApiInternalerror,
            -9 => InstProxyError::ApplicationAlreadyInstalled,
            -10 => InstProxyError::ApplicationMoveFailed,
            -11 => InstProxyError::ApplicationSinfCaptureFailed,
            -12 => InstProxyError::ApplicationSandboxFailed,
            -13 => InstProxyError::ApplicationVerificationFailed,
            -14 => InstProxyError::ArchiveDestructionFailed,
            -15 => InstProxyError::BundleVerificationFailed,
            -16 => InstProxyError::CarrierBundleCopyFailed,
            -17 => InstProxyError::CarrierBundleDirectoryCreationFailed,
            -18 => InstProxyError::CarrierBundleMissingSupportedSims,
            -19 => InstProxyError::CommCenterNotificationFailed,
            -20 => InstProxyError::ContainerCreationFailed,
            -21 => InstProxyError::ContainerP0wnFailed,
            -22 => InstProxyError::ContainerRemovalFailed,
            -23 => InstProxyError::EmbeddedProfileInstallFailed,
            -24 => InstProxyError::ExecutableTwiddleFailed,
            -25 => InstProxyError::ExistenceCheckFailed,
            -26 => InstProxyError::InstallMapUpdateFailed,
            -27 => InstProxyError::ManifestCaptureFailed,
            -28 => InstProxyError::MapGenerationFailed,
            -29 => InstProxyError::MissingBundleExecutable,
            -30 => InstProxyError::MissingBundleIdentifier,
            -31 => InstProxyError::MissingBundlePath,
            -32 => InstProxyError::MissingContainer,
            -33 => InstProxyError::NotificationFailed,
            -34 => InstProxyError::PackageExtractionFailed,
            -35 => InstProxyError::PackageInspectionFailed,
            -36 => InstProxyError::PackageMoveFailed,
            -37 => InstProxyError::PathConversionFailed,
            -38 => InstProxyError::RestoreConversionFailed,
            -39 => InstProxyError::SeatbeltProfileRemovalFailed,
            -40 => InstProxyError::StageCreationFailed,
            -41 => InstProxyError::SymlinkFailed,
            -42 => InstProxyError::UnknownCommand,
            -43 => InstProxyError::ItunesArtworkCaptureFailed,
            -44 => InstProxyError::ItunesMetadataCaptureFailed,
            -45 => InstProxyError::DeviceOsVersionTooLow,
            -46 => InstProxyError::DeviceFamilyNotSupported,
            -47 => InstProxyError::PackagePatchFailed,
            -48 => InstProxyError::IncorrectArchitecture,
            -49 => InstProxyError::PluginCopyFailed,
            -50 => InstProxyError::BreadcrumbFailed,
            -51 => InstProxyError::BreadcrumbUnlockFailed,
            -52 => InstProxyError::GeoJsonCaptureFailed,
            -53 => InstProxyError::NewsstandArtworkCaptureFailed,
            -54 => InstProxyError::MissingCommand,
            -55 => InstProxyError::NotEntitled,
            -56 => InstProxyError::MissingPackagePath,
            -57 => InstProxyError::MissingContainerPath,
            -58 => InstProxyError::MissingApplicationIdentifier,
            -59 => InstProxyError::MissingAttributeValue,
            -60 => InstProxyError::LookupFailed,
            -61 => InstProxyError::DictCreationFailed,
            -62 => InstProxyError::InstallProhibited,
            -63 => InstProxyError::UninstallProhibited,
            -64 => InstProxyError::MissingBundleVersion,
            _ => InstProxyError::UnknownError,
        }
    }
            
}

impl From<InstProxyError> for String {
    fn from(e: InstProxyError) -> String {
        match e {
            InstProxyError::Success => "Success".to_string(),
            InstProxyError::InvalidArg => "InvalidArg".to_string(),
            InstProxyError::PlistError => "PlistError".to_string(),
            InstProxyError::ConnFailed => "ConnFailed".to_string(),
            InstProxyError::OpInProgress => "OpInProgress".to_string(),
            InstProxyError::OpFailed => "OpFailed".to_string(),
            InstProxyError::RecieveTimeout => "RecieveTimeout".to_string(),
            InstProxyError::AlreadyArchived => "AlreadyArchived".to_string(),
            InstProxyError::ApiInternalerror => "ApiInternalerror".to_string(),
            InstProxyError::ApplicationAlreadyInstalled => "ApplicationAlreadyInstalled".to_string(),
            InstProxyError::ApplicationMoveFailed => "ApplicationMoveFailed".to_string(),
            InstProxyError::ApplicationSinfCaptureFailed => "ApplicationSinfCaptureFailed".to_string(),
            InstProxyError::ApplicationSandboxFailed => "ApplicationSandboxFailed".to_string(),
            InstProxyError::ApplicationVerificationFailed => "ApplicationVerificationFailed".to_string(),
            InstProxyError::ArchiveDestructionFailed => "ArchiveDestructionFailed".to_string(),
            InstProxyError::BundleVerificationFailed => "BundleVerficationFailed".to_string(),
            InstProxyError::CarrierBundleCopyFailed => "CarrierBundleCopyFailed".to_string(),
            InstProxyError::CarrierBundleDirectoryCreationFailed => "CarrierBundleDirectoryCreationFailed".to_string(),
            InstProxyError::CarrierBundleMissingSupportedSims => "CarrierBundleMissingSupportedSims".to_string(),
            InstProxyError::CommCenterNotificationFailed => "CommCenterNotificationFailed".to_string(),
            InstProxyError::ContainerCreationFailed => "ContainerCreationFailed".to_string(),
            InstProxyError::ContainerP0wnFailed => "ContainerP0wnFailed".to_string(),
            InstProxyError::ContainerRemovalFailed => "ContainerRemovalFailed".to_string(),
            InstProxyError::EmbeddedProfileInstallFailed => "EmbeddedProfileInstallFailed".to_string(),
            InstProxyError::ExecutableTwiddleFailed => "ExecutableTwiddleFailed".to_string(),
            InstProxyError::ExistenceCheckFailed => "ExistenceCheckFailed".to_string(),
            InstProxyError::InstallMapUpdateFailed => "InstallMapUpdateFailed".to_string(),
            InstProxyError::ManifestCaptureFailed => "ManifestCaptureFailed".to_string(),
            InstProxyError::MapGenerationFailed => "MapGenerationFailed".to_string(),
            InstProxyError::MissingBundleExecutable => "MissingBundleExecutable".to_string(),
            InstProxyError::MissingBundleIdentifier => "MissingBundleIdentifier".to_string(),
            InstProxyError::MissingBundlePath => "MissingBundlePath".to_string(),
            InstProxyError::MissingContainer => "MissingContainer".to_string(),
            InstProxyError::NotificationFailed => "NotificationFailed".to_string(),
            InstProxyError::PackageExtractionFailed => "PackageExtractionFailed".to_string(),
            InstProxyError::PackageInspectionFailed => "PackageInspectionFailed".to_string(),
            InstProxyError::PackageMoveFailed => "PackageMoveFailed".to_string(),
            InstProxyError::PathConversionFailed => "PathConversionFailed".to_string(),
            InstProxyError::RestoreConversionFailed => "RestoreConversionFailed".to_string(),
            InstProxyError::SeatbeltProfileRemovalFailed => "SeatbeltProfileRemovalFailed".to_string(),
            InstProxyError::StageCreationFailed => "StageCreationFailed".to_string(),
            InstProxyError::SymlinkFailed => "SymlinkFailed".to_string(),
            InstProxyError::UnknownCommand => "UnknownCommand".to_string(),
            InstProxyError::ItunesArtworkCaptureFailed => "ItunesArtworkCaptureFailed".to_string(),
            InstProxyError::ItunesMetadataCaptureFailed => "ItunesMetadataCaptureFailed".to_string(),
            InstProxyError::DeviceOsVersionTooLow => "DeviceOsVersionTooLow".to_string(),
            InstProxyError::DeviceFamilyNotSupported => "DeviceFamilyNotSupported".to_string(),
            InstProxyError::PackagePatchFailed => "PackagePatchFailed".to_string(),
            InstProxyError::IncorrectArchitecture => "IncorrectArchitecture".to_string(),
            InstProxyError::PluginCopyFailed => "PluginCopyFailed".to_string(),
            InstProxyError::BreadcrumbFailed => "BreadcrumbFailed".to_string(),
            InstProxyError::BreadcrumbUnlockFailed => "BreadcrumbUnlockFailed".to_string(),
            InstProxyError::GeoJsonCaptureFailed => "GeoJsonCaptureFailed".to_string(),
            InstProxyError::NewsstandArtworkCaptureFailed => "NewsstandArtworkCaptureFailed".to_string(),
            InstProxyError::MissingCommand => "MissingCommand".to_string(),
            InstProxyError::NotEntitled => "NotEntitled".to_string(),
            InstProxyError::MissingPackagePath => "MissingPackagePath".to_string(),
            InstProxyError::MissingContainerPath => "MissingContainerPath".to_string(),
            InstProxyError::MissingApplicationIdentifier => "MissingApplicationIdentifier".to_string(),
            InstProxyError::MissingBundleVersion => "MissingBundleVersion".to_string(),
            InstProxyError::UninstallProhibited => "UninstallProhibited".to_string(),
            InstProxyError::UnknownError => "UnknownError".to_string(),
            InstProxyError::MissingAttributeValue => "MissingAttributeValue".to_string(),
            InstProxyError::LookupFailed => "LookupFailed".to_string(),
            InstProxyError::DictCreationFailed => "DictCreationFailed".to_string(),
            InstProxyError::InstallProhibited => "InstallProhibited".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum DebugServerError {
    Success,
    InvalidArg,
    MuxError,
    SslError,
    ResponseError,
    Timeout,
    UnknownError
}

impl From<i32> for DebugServerError {
    fn from(value: i32) -> DebugServerError {
        match value {
            0 => DebugServerError::Success,
            1 => DebugServerError::InvalidArg,
            2 => DebugServerError::MuxError,
            3 => DebugServerError::SslError,
            4 => DebugServerError::ResponseError,
            5 => DebugServerError::Timeout,
            _ => DebugServerError::UnknownError
        }
    }
}

impl From<DebugServerError> for String {
    fn from(value: DebugServerError) -> String {
        match value {
            DebugServerError::Success => "Success".to_string(),
            DebugServerError::InvalidArg => "InvalidArg".to_string(),
            DebugServerError::MuxError => "MuxError".to_string(),
            DebugServerError::SslError => "SslError".to_string(),
            DebugServerError::ResponseError => "ResponseError".to_string(),
            DebugServerError::Timeout => "Timeout".to_string(),
            DebugServerError::UnknownError => "UnknownError".to_string(),
        }
    }
}
