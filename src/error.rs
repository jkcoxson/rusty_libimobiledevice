// jkcoxson
// Enums for libimobiledevice's error codes

#[derive(PartialEq, Eq, Debug)]
pub enum LockdowndError {
    Success,
    InvalidArg,
    InvalidConf,
    PlistError,
    PairingFailed,
    SslError,
    DictError,
    ReceiveTimeout,
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
    // Internal errors
    MissingObjectDepenency,
}

impl std::error::Error for LockdowndError {}

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
            -7 => LockdowndError::ReceiveTimeout,
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
            -100 => LockdowndError::MissingObjectDepenency,
            _ => LockdowndError::UnknownError,
        }
    }
}

impl std::fmt::Display for LockdowndError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LockdowndError::Success => "Success",
            LockdowndError::InvalidArg => "InvalidArg",
            LockdowndError::InvalidConf => "InvalidConf",
            LockdowndError::PlistError => "PlistError",
            LockdowndError::PairingFailed => "PairingFailed",
            LockdowndError::SslError => "SslError",
            LockdowndError::DictError => "DictError",
            LockdowndError::ReceiveTimeout => "ReceiveTimeout",
            LockdowndError::MuxError => "MuxError",
            LockdowndError::NoRunningSession => "NoRunningSession",
            LockdowndError::InvalidResponse => "InvalidResponse",
            LockdowndError::MissingKey => "MissingKey",
            LockdowndError::MissingValue => "MissingValue",
            LockdowndError::GetProhibited => "GetProhibited",
            LockdowndError::SetProhibited => "SetProhibited",
            LockdowndError::RemoveProhibited => "RemoveProhibited",
            LockdowndError::ImmutableValue => "ImmutableValue",
            LockdowndError::PasswordProtected => "PasswordProtected",
            LockdowndError::UserDeniedPairing => "UserDeniedPairing",
            LockdowndError::PairingDialogueRepsonsePending => "PairingDialogueRepsonsePending",
            LockdowndError::MissingHostId => "MissingHostId",
            LockdowndError::InvalidHostId => "InvalidHostId",
            LockdowndError::SessionActive => "SessionActive",
            LockdowndError::SessionInactive => "SessionInactive",
            LockdowndError::MissingSessionId => "MissingSessionId",
            LockdowndError::InvalidSessionId => "InvalidSessionId",
            LockdowndError::MissingService => "MissingService",
            LockdowndError::InvalidService => "InvalidService",
            LockdowndError::ServiceLimit => "ServiceLimit",
            LockdowndError::MissingPairRecord => "MissingPairRecord",
            LockdowndError::SavePairRecordFailed => "SavePairRecordFailed",
            LockdowndError::InvalidPairRecord => "InvalidPairRecord",
            LockdowndError::InvalidActivationRecord => "InvalidActivationRecord",
            LockdowndError::MissingActivationRecord => "MissingActivationRecord",
            LockdowndError::ServiceProhibited => "ServiceProhibited",
            LockdowndError::EscrowLocked => "EscrowLocked",
            LockdowndError::PairingProhibitedOverThisConnection => {
                "PairingProhibitedOverThisConnection"
            }
            LockdowndError::FmipProtected => "FmipProtected",
            LockdowndError::McProtected => "McProtected",
            LockdowndError::McChallengeRequired => "McChallengeRequired",
            LockdowndError::UnknownError => "UnknownError",
            LockdowndError::MissingObjectDepenency => "MissingObjectDepenency",
        })
    }
}

impl From<LockdowndError> for String {
    fn from(e: LockdowndError) -> String {
        e.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
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

impl std::error::Error for IdeviceError {}

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

impl std::fmt::Display for IdeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            IdeviceError::Success => "Success",
            IdeviceError::InvalidArg => "InvalidArg",
            IdeviceError::UnknownError => "UnknownError",
            IdeviceError::NoDevice => "NoDevice",
            IdeviceError::NotEnoughData => "NotEnoughData",
            IdeviceError::ConnRefused => "ConnRefused",
            IdeviceError::SslError => "SslError",
            IdeviceError::Timeout => "Timeout",
        })
    }
}

impl From<IdeviceError> for String {
    fn from(e: IdeviceError) -> String {
        e.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
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

impl std::error::Error for UserPrefError {}

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

impl std::fmt::Display for UserPrefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UserPrefError::Success => "Success",
            UserPrefError::InvalidArg => "InvalidArg",
            UserPrefError::NoEnt => "NoEnt",
            UserPrefError::InvalidConf => "InvalidConf",
            UserPrefError::SslError => "SslError",
            UserPrefError::ReadError => "ReadError",
            UserPrefError::WriteError => "WriteError",
            UserPrefError::UnknownError => "UnknownError",
        })
    }
}

impl From<UserPrefError> for String {
    fn from(e: UserPrefError) -> String {
        e.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum PropertyListServiceError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    NotEnoughData,
    UnknownError,
}

impl std::error::Error for PropertyListServiceError {}

impl From<i32> for PropertyListServiceError {
    fn from(e: i32) -> PropertyListServiceError {
        match e {
            0 => PropertyListServiceError::Success,
            -1 => PropertyListServiceError::InvalidArg,
            -2 => PropertyListServiceError::PlistError,
            -3 => PropertyListServiceError::MuxError,
            -4 => PropertyListServiceError::SslError,
            -5 => PropertyListServiceError::ReceiveTimeout,
            -6 => PropertyListServiceError::NotEnoughData,
            _ => PropertyListServiceError::UnknownError,
        }
    }
}

impl std::fmt::Display for PropertyListServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PropertyListServiceError::Success => "Success",
            PropertyListServiceError::InvalidArg => "InvalidArg",
            PropertyListServiceError::PlistError => "PlistError",
            PropertyListServiceError::MuxError => "MuxError",
            PropertyListServiceError::SslError => "SslError",
            PropertyListServiceError::ReceiveTimeout => "ReceiveTimeout",
            PropertyListServiceError::NotEnoughData => "NotEnoughData",
            PropertyListServiceError::UnknownError => "UnknownError",
        })
    }
}

impl From<PropertyListServiceError> for String {
    fn from(e: PropertyListServiceError) -> String {
        e.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
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

impl std::error::Error for ServiceError {}

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

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServiceError::Success => "Success",
            ServiceError::InvalidArg => "InvalidArg",
            ServiceError::MuxError => "MuxError",
            ServiceError::SslError => "SslError",
            ServiceError::StartServiceError => "StartServiceError",
            ServiceError::NotEnoughData => "NotEnoughData",
            ServiceError::Timeout => "Timeout",
            ServiceError::UnknownError => "UnknownError",
        })
    }
}

impl From<ServiceError> for String {
    fn from(e: ServiceError) -> String {
        e.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum InstProxyError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    OpInProgress,
    OpFailed,
    ReceiveTimeout,
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
    // Internal
    MissingObjectDepenency,
}

impl std::error::Error for InstProxyError {}

impl From<i32> for InstProxyError {
    fn from(e: i32) -> InstProxyError {
        match e {
            0 => InstProxyError::Success,
            -1 => InstProxyError::InvalidArg,
            -2 => InstProxyError::PlistError,
            -3 => InstProxyError::ConnFailed,
            -4 => InstProxyError::OpInProgress,
            -5 => InstProxyError::OpFailed,
            -6 => InstProxyError::ReceiveTimeout,
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
            -100 => InstProxyError::MissingObjectDepenency,
            _ => InstProxyError::UnknownError,
        }
    }
}

impl std::fmt::Display for InstProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InstProxyError::Success => "Success",
            InstProxyError::InvalidArg => "InvalidArg",
            InstProxyError::PlistError => "PlistError",
            InstProxyError::ConnFailed => "ConnFailed",
            InstProxyError::OpInProgress => "OpInProgress",
            InstProxyError::OpFailed => "OpFailed",
            InstProxyError::ReceiveTimeout => "ReceiveTimeout",
            InstProxyError::AlreadyArchived => "AlreadyArchived",
            InstProxyError::ApiInternalerror => "ApiInternalerror",
            InstProxyError::ApplicationAlreadyInstalled => "ApplicationAlreadyInstalled",
            InstProxyError::ApplicationMoveFailed => "ApplicationMoveFailed",
            InstProxyError::ApplicationSinfCaptureFailed => "ApplicationSinfCaptureFailed",
            InstProxyError::ApplicationSandboxFailed => "ApplicationSandboxFailed",
            InstProxyError::ApplicationVerificationFailed => "ApplicationVerificationFailed",
            InstProxyError::ArchiveDestructionFailed => "ArchiveDestructionFailed",
            InstProxyError::BundleVerificationFailed => "BundleVerficationFailed",
            InstProxyError::CarrierBundleCopyFailed => "CarrierBundleCopyFailed",
            InstProxyError::CarrierBundleDirectoryCreationFailed => {
                "CarrierBundleDirectoryCreationFailed"
            }
            InstProxyError::CarrierBundleMissingSupportedSims => {
                "CarrierBundleMissingSupportedSims"
            }
            InstProxyError::CommCenterNotificationFailed => "CommCenterNotificationFailed",
            InstProxyError::ContainerCreationFailed => "ContainerCreationFailed",
            InstProxyError::ContainerP0wnFailed => "ContainerP0wnFailed",
            InstProxyError::ContainerRemovalFailed => "ContainerRemovalFailed",
            InstProxyError::EmbeddedProfileInstallFailed => "EmbeddedProfileInstallFailed",
            InstProxyError::ExecutableTwiddleFailed => "ExecutableTwiddleFailed",
            InstProxyError::ExistenceCheckFailed => "ExistenceCheckFailed",
            InstProxyError::InstallMapUpdateFailed => "InstallMapUpdateFailed",
            InstProxyError::ManifestCaptureFailed => "ManifestCaptureFailed",
            InstProxyError::MapGenerationFailed => "MapGenerationFailed",
            InstProxyError::MissingBundleExecutable => "MissingBundleExecutable",
            InstProxyError::MissingBundleIdentifier => "MissingBundleIdentifier",
            InstProxyError::MissingBundlePath => "MissingBundlePath",
            InstProxyError::MissingContainer => "MissingContainer",
            InstProxyError::NotificationFailed => "NotificationFailed",
            InstProxyError::PackageExtractionFailed => "PackageExtractionFailed",
            InstProxyError::PackageInspectionFailed => "PackageInspectionFailed",
            InstProxyError::PackageMoveFailed => "PackageMoveFailed",
            InstProxyError::PathConversionFailed => "PathConversionFailed",
            InstProxyError::RestoreConversionFailed => "RestoreConversionFailed",
            InstProxyError::SeatbeltProfileRemovalFailed => "SeatbeltProfileRemovalFailed",
            InstProxyError::StageCreationFailed => "StageCreationFailed",
            InstProxyError::SymlinkFailed => "SymlinkFailed",
            InstProxyError::UnknownCommand => "UnknownCommand",
            InstProxyError::ItunesArtworkCaptureFailed => "ItunesArtworkCaptureFailed",
            InstProxyError::ItunesMetadataCaptureFailed => "ItunesMetadataCaptureFailed",
            InstProxyError::DeviceOsVersionTooLow => "DeviceOsVersionTooLow",
            InstProxyError::DeviceFamilyNotSupported => "DeviceFamilyNotSupported",
            InstProxyError::PackagePatchFailed => "PackagePatchFailed",
            InstProxyError::IncorrectArchitecture => "IncorrectArchitecture",
            InstProxyError::PluginCopyFailed => "PluginCopyFailed",
            InstProxyError::BreadcrumbFailed => "BreadcrumbFailed",
            InstProxyError::BreadcrumbUnlockFailed => "BreadcrumbUnlockFailed",
            InstProxyError::GeoJsonCaptureFailed => "GeoJsonCaptureFailed",
            InstProxyError::NewsstandArtworkCaptureFailed => "NewsstandArtworkCaptureFailed",
            InstProxyError::MissingCommand => "MissingCommand",
            InstProxyError::NotEntitled => "NotEntitled",
            InstProxyError::MissingPackagePath => "MissingPackagePath",
            InstProxyError::MissingContainerPath => "MissingContainerPath",
            InstProxyError::MissingApplicationIdentifier => "MissingApplicationIdentifier",
            InstProxyError::MissingBundleVersion => "MissingBundleVersion",
            InstProxyError::UninstallProhibited => "UninstallProhibited",
            InstProxyError::UnknownError => "UnknownError",
            InstProxyError::MissingAttributeValue => "MissingAttributeValue",
            InstProxyError::LookupFailed => "LookupFailed",
            InstProxyError::DictCreationFailed => "DictCreationFailed",
            InstProxyError::InstallProhibited => "InstallProhibited",
            InstProxyError::MissingObjectDepenency => "MissingObjectDependency",
        })
    }
}

impl From<InstProxyError> for String {
    fn from(e: InstProxyError) -> String {
        e.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum DebugServerError {
    Success,
    InvalidArg,
    MuxError,
    SslError,
    ResponseError,
    Timeout,
    UnknownError,
}

impl std::error::Error for DebugServerError {}

impl From<i32> for DebugServerError {
    fn from(value: i32) -> DebugServerError {
        match value {
            0 => DebugServerError::Success,
            1 => DebugServerError::InvalidArg,
            2 => DebugServerError::MuxError,
            3 => DebugServerError::SslError,
            4 => DebugServerError::ResponseError,
            5 => DebugServerError::Timeout,
            _ => DebugServerError::UnknownError,
        }
    }
}

impl std::fmt::Display for DebugServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DebugServerError::Success => "Success",
            DebugServerError::InvalidArg => "InvalidArg",
            DebugServerError::MuxError => "MuxError",
            DebugServerError::SslError => "SslError",
            DebugServerError::ResponseError => "ResponseError",
            DebugServerError::Timeout => "Timeout",
            DebugServerError::UnknownError => "UnknownError",
        })
    }
}

impl From<DebugServerError> for String {
    fn from(value: DebugServerError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum WebInspectorError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    NotEnoughData,
    UnknownError,
}

impl std::error::Error for WebInspectorError {}

impl From<i32> for WebInspectorError {
    fn from(value: i32) -> WebInspectorError {
        match value {
            0 => WebInspectorError::Success,
            -1 => WebInspectorError::InvalidArg,
            -2 => WebInspectorError::PlistError,
            -3 => WebInspectorError::MuxError,
            -4 => WebInspectorError::SslError,
            -5 => WebInspectorError::ReceiveTimeout,
            -6 => WebInspectorError::NotEnoughData,
            _ => WebInspectorError::UnknownError,
        }
    }
}

impl std::fmt::Display for WebInspectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            WebInspectorError::Success => "Success",
            WebInspectorError::InvalidArg => "InvalidArg",
            WebInspectorError::PlistError => "PlistError",
            WebInspectorError::MuxError => "MuxError",
            WebInspectorError::SslError => "SslError",
            WebInspectorError::ReceiveTimeout => "ReceiveTimeout",
            WebInspectorError::NotEnoughData => "NotEnoughData",
            WebInspectorError::UnknownError => "UnknownError",
        })
    }
}

impl From<WebInspectorError> for String {
    fn from(value: WebInspectorError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum SyslogRelayError {
    Success,
    InvalidArg,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    UnknownError,
}

impl std::error::Error for SyslogRelayError {}

impl From<i32> for SyslogRelayError {
    fn from(value: i32) -> SyslogRelayError {
        match value {
            0 => SyslogRelayError::Success,
            -1 => SyslogRelayError::InvalidArg,
            -2 => SyslogRelayError::MuxError,
            -3 => SyslogRelayError::SslError,
            -4 => SyslogRelayError::NotEnoughData,
            -5 => SyslogRelayError::Timeout,
            _ => SyslogRelayError::UnknownError,
        }
    }
}

impl std::fmt::Display for SyslogRelayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SyslogRelayError::Success => "Success",
            SyslogRelayError::InvalidArg => "InvalidArg",
            SyslogRelayError::MuxError => "MuxError",
            SyslogRelayError::SslError => "SslError",
            SyslogRelayError::NotEnoughData => "NotEnoughData",
            SyslogRelayError::Timeout => "Timeout",
            SyslogRelayError::UnknownError => "UnknownError",
        })
    }
}

impl From<SyslogRelayError> for String {
    fn from(value: SyslogRelayError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ScreenshotrError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    BadVersion,
    UnknownError,
}

impl std::error::Error for ScreenshotrError {}

impl From<i32> for ScreenshotrError {
    fn from(value: i32) -> ScreenshotrError {
        match value {
            0 => ScreenshotrError::Success,
            -1 => ScreenshotrError::InvalidArg,
            -2 => ScreenshotrError::PlistError,
            -3 => ScreenshotrError::MuxError,
            -4 => ScreenshotrError::SslError,
            -5 => ScreenshotrError::ReceiveTimeout,
            -6 => ScreenshotrError::BadVersion,
            _ => ScreenshotrError::UnknownError,
        }
    }
}

impl std::fmt::Display for ScreenshotrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ScreenshotrError::Success => "Success",
            ScreenshotrError::InvalidArg => "InvalidArg",
            ScreenshotrError::PlistError => "PlistError",
            ScreenshotrError::MuxError => "MuxError",
            ScreenshotrError::SslError => "SslError",
            ScreenshotrError::ReceiveTimeout => "ReceiveTimeout",
            ScreenshotrError::BadVersion => "BadVersion",
            ScreenshotrError::UnknownError => "UnknownError",
        })
    }
}

impl From<ScreenshotrError> for String {
    fn from(value: ScreenshotrError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum SbservicesError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    UnknownError,
}

impl std::error::Error for SbservicesError {}

impl From<i32> for SbservicesError {
    fn from(value: i32) -> SbservicesError {
        match value {
            0 => SbservicesError::Success,
            -1 => SbservicesError::InvalidArg,
            -2 => SbservicesError::PlistError,
            -3 => SbservicesError::ConnFailed,
            _ => SbservicesError::UnknownError,
        }
    }
}

impl std::fmt::Display for SbservicesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SbservicesError::Success => "Success",
            SbservicesError::InvalidArg => "InvalidArg",
            SbservicesError::PlistError => "PlistError",
            SbservicesError::ConnFailed => "ConnFailed",
            SbservicesError::UnknownError => "UnknownError",
        })
    }
}

impl From<SbservicesError> for String {
    fn from(value: SbservicesError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ReverseProxyError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    UnknownError,
}

impl std::error::Error for ReverseProxyError {}

impl From<i32> for ReverseProxyError {
    fn from(value: i32) -> ReverseProxyError {
        match value {
            0 => ReverseProxyError::Success,
            -1 => ReverseProxyError::InvalidArg,
            -2 => ReverseProxyError::PlistError,
            -3 => ReverseProxyError::MuxError,
            -4 => ReverseProxyError::SslError,
            -5 => ReverseProxyError::NotEnoughData,
            -6 => ReverseProxyError::Timeout,
            _ => ReverseProxyError::UnknownError,
        }
    }
}

impl std::fmt::Display for ReverseProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ReverseProxyError::Success => "Success",
            ReverseProxyError::InvalidArg => "InvalidArg",
            ReverseProxyError::PlistError => "PlistError",
            ReverseProxyError::MuxError => "MuxError",
            ReverseProxyError::SslError => "SslError",
            ReverseProxyError::NotEnoughData => "NotEnoughData",
            ReverseProxyError::Timeout => "Timeout",
            ReverseProxyError::UnknownError => "UnknownError",
        })
    }
}

impl From<ReverseProxyError> for String {
    fn from(value: ReverseProxyError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum RestoredError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    NotEnoughData,
    ReceiveTimeout,
    UnknownError,
}

impl std::error::Error for RestoredError {}

impl From<i32> for RestoredError {
    fn from(value: i32) -> RestoredError {
        match value {
            0 => RestoredError::Success,
            -1 => RestoredError::InvalidArg,
            -2 => RestoredError::PlistError,
            -3 => RestoredError::MuxError,
            -4 => RestoredError::NotEnoughData,
            -5 => RestoredError::ReceiveTimeout,
            _ => RestoredError::UnknownError,
        }
    }
}

impl std::fmt::Display for RestoredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RestoredError::Success => "Success",
            RestoredError::InvalidArg => "InvalidArg",
            RestoredError::PlistError => "PlistError",
            RestoredError::MuxError => "MuxError",
            RestoredError::NotEnoughData => "NotEnoughData",
            RestoredError::ReceiveTimeout => "ReceiveTimeout",
            RestoredError::UnknownError => "UnknownError",
        })
    }
}

impl From<RestoredError> for String {
    fn from(value: RestoredError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum PreboardError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    OpInProgress,
    UnknownError,
}

impl std::error::Error for PreboardError {}

impl From<i32> for PreboardError {
    fn from(value: i32) -> PreboardError {
        match value {
            0 => PreboardError::Success,
            -1 => PreboardError::InvalidArg,
            -2 => PreboardError::PlistError,
            -3 => PreboardError::MuxError,
            -4 => PreboardError::SslError,
            -5 => PreboardError::NotEnoughData,
            -6 => PreboardError::Timeout,
            -10 => PreboardError::OpInProgress,
            _ => PreboardError::UnknownError,
        }
    }
}

impl std::fmt::Display for PreboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PreboardError::Success => "Success",
            PreboardError::InvalidArg => "InvalidArg",
            PreboardError::PlistError => "PlistError",
            PreboardError::MuxError => "MuxError",
            PreboardError::SslError => "SslError",
            PreboardError::NotEnoughData => "NotEnoughData",
            PreboardError::Timeout => "Timeout",
            PreboardError::OpInProgress => "OpInProgress",
            PreboardError::UnknownError => "UnknownError",
        })
    }
}

impl From<PreboardError> for String {
    fn from(value: PreboardError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum NpError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    UnknownError,
}

impl std::error::Error for NpError {}

impl From<i32> for NpError {
    fn from(value: i32) -> NpError {
        match value {
            0 => NpError::Success,
            -1 => NpError::InvalidArg,
            -2 => NpError::PlistError,
            -3 => NpError::ConnFailed,
            _ => NpError::UnknownError,
        }
    }
}

impl std::fmt::Display for NpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NpError::Success => "Success",
            NpError::InvalidArg => "InvalidArg",
            NpError::PlistError => "PlistError",
            NpError::ConnFailed => "ConnFailed",
            NpError::UnknownError => "UnknownError",
        })
    }
}

impl From<NpError> for String {
    fn from(value: NpError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MobileSyncError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    BadVersion,
    SyncRefused,
    Cancelled,
    WrongDirection,
    NotReady,
    UnknownError,
}

impl std::error::Error for MobileSyncError {}

impl From<i32> for MobileSyncError {
    fn from(value: i32) -> MobileSyncError {
        match value {
            0 => MobileSyncError::Success,
            -1 => MobileSyncError::InvalidArg,
            -2 => MobileSyncError::PlistError,
            -3 => MobileSyncError::MuxError,
            -4 => MobileSyncError::SslError,
            -5 => MobileSyncError::ReceiveTimeout,
            -6 => MobileSyncError::BadVersion,
            -7 => MobileSyncError::SyncRefused,
            -8 => MobileSyncError::Cancelled,
            -9 => MobileSyncError::WrongDirection,
            -10 => MobileSyncError::NotReady,
            _ => MobileSyncError::UnknownError,
        }
    }
}

impl std::fmt::Display for MobileSyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MobileSyncError::Success => "Success",
            MobileSyncError::InvalidArg => "InvalidArg",
            MobileSyncError::PlistError => "PlistError",
            MobileSyncError::MuxError => "MuxError",
            MobileSyncError::SslError => "SslError",
            MobileSyncError::ReceiveTimeout => "ReceiveTimeout",
            MobileSyncError::BadVersion => "BadVersion",
            MobileSyncError::SyncRefused => "SyncRefused",
            MobileSyncError::Cancelled => "Cancelled",
            MobileSyncError::WrongDirection => "WrongDirection",
            MobileSyncError::NotReady => "NotReady",
            MobileSyncError::UnknownError => "UnknownError",
        })
    }
}

impl From<MobileSyncError> for String {
    fn from(value: MobileSyncError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MobileBackup2Error {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    BadVersion,
    ReplyNotOk,
    NoCommonVersion,
    UnknownError,
}

impl std::error::Error for MobileBackup2Error {}

impl From<i32> for MobileBackup2Error {
    fn from(value: i32) -> MobileBackup2Error {
        match value {
            0 => MobileBackup2Error::Success,
            -1 => MobileBackup2Error::InvalidArg,
            -2 => MobileBackup2Error::PlistError,
            -3 => MobileBackup2Error::MuxError,
            -4 => MobileBackup2Error::SslError,
            -5 => MobileBackup2Error::ReceiveTimeout,
            -6 => MobileBackup2Error::BadVersion,
            -7 => MobileBackup2Error::ReplyNotOk,
            -8 => MobileBackup2Error::NoCommonVersion,
            _ => MobileBackup2Error::UnknownError,
        }
    }
}

impl std::fmt::Display for MobileBackup2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MobileBackup2Error::Success => "Success",
            MobileBackup2Error::InvalidArg => "InvalidArg",
            MobileBackup2Error::PlistError => "PlistError",
            MobileBackup2Error::MuxError => "MuxError",
            MobileBackup2Error::SslError => "SslError",
            MobileBackup2Error::ReceiveTimeout => "ReceiveTimeout",
            MobileBackup2Error::BadVersion => "BadVersion",
            MobileBackup2Error::ReplyNotOk => "ReplyNotOk",
            MobileBackup2Error::NoCommonVersion => "NoCommonVersion",
            MobileBackup2Error::UnknownError => "UnknownError",
        })
    }
}

impl From<MobileBackup2Error> for String {
    fn from(value: MobileBackup2Error) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MobileBackupError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    BadVersion,
    ReplyNotOk,
    UnknownError,
}

impl std::error::Error for MobileBackupError {}

impl From<i32> for MobileBackupError {
    fn from(value: i32) -> MobileBackupError {
        match value {
            0 => MobileBackupError::Success,
            -1 => MobileBackupError::InvalidArg,
            -2 => MobileBackupError::PlistError,
            -3 => MobileBackupError::MuxError,
            -4 => MobileBackupError::SslError,
            -5 => MobileBackupError::ReceiveTimeout,
            -6 => MobileBackupError::BadVersion,
            -7 => MobileBackupError::ReplyNotOk,
            _ => MobileBackupError::UnknownError,
        }
    }
}

impl std::fmt::Display for MobileBackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MobileBackupError::Success => "Success",
            MobileBackupError::InvalidArg => "InvalidArg",
            MobileBackupError::PlistError => "PlistError",
            MobileBackupError::MuxError => "MuxError",
            MobileBackupError::SslError => "SslError",
            MobileBackupError::ReceiveTimeout => "ReceiveTimeout",
            MobileBackupError::BadVersion => "BadVersion",
            MobileBackupError::ReplyNotOk => "ReplyNotOk",
            MobileBackupError::UnknownError => "UnknownError",
        })
    }
}

impl From<MobileBackupError> for String {
    fn from(value: MobileBackupError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MobileActivationError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    UnknownRequest,
    RequestFailed,
    UnknownError,
}

impl std::error::Error for MobileActivationError {}

impl From<i32> for MobileActivationError {
    fn from(value: i32) -> MobileActivationError {
        match value {
            0 => MobileActivationError::Success,
            -1 => MobileActivationError::InvalidArg,
            -2 => MobileActivationError::PlistError,
            -3 => MobileActivationError::MuxError,
            -4 => MobileActivationError::UnknownRequest,
            -5 => MobileActivationError::RequestFailed,
            _ => MobileActivationError::UnknownError,
        }
    }
}

impl std::fmt::Display for MobileActivationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MobileActivationError::Success => "Success",
            MobileActivationError::InvalidArg => "InvalidArg",
            MobileActivationError::PlistError => "PlistError",
            MobileActivationError::MuxError => "MuxError",
            MobileActivationError::UnknownRequest => "UnknownRequest",
            MobileActivationError::RequestFailed => "RequestFailed",
            MobileActivationError::UnknownError => "UnknownError",
        })
    }
}

impl From<MobileActivationError> for String {
    fn from(value: MobileActivationError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MobileImageMounterError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    CommandFailed,
    DeviceLocked,
    UnknownError,
    // Internal errors
    DmgNotFound,
    SignatureNotFound,
    MissingObjectDepenency,
}

impl std::error::Error for MobileImageMounterError {}

impl From<i32> for MobileImageMounterError {
    fn from(value: i32) -> MobileImageMounterError {
        match value {
            0 => MobileImageMounterError::Success,
            -1 => MobileImageMounterError::InvalidArg,
            -2 => MobileImageMounterError::PlistError,
            -3 => MobileImageMounterError::ConnFailed,
            -4 => MobileImageMounterError::CommandFailed,
            -5 => MobileImageMounterError::DeviceLocked,
            -100 => MobileImageMounterError::DmgNotFound,
            -101 => MobileImageMounterError::SignatureNotFound,
            -102 => MobileImageMounterError::MissingObjectDepenency,
            _ => MobileImageMounterError::UnknownError,
        }
    }
}

impl std::fmt::Display for MobileImageMounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MobileImageMounterError::Success => "Success",
            MobileImageMounterError::InvalidArg => "InvalidArg",
            MobileImageMounterError::PlistError => "PlistError",
            MobileImageMounterError::ConnFailed => "ConnFailed",
            MobileImageMounterError::CommandFailed => "CommandFailed",
            MobileImageMounterError::DeviceLocked => "DeviceLocked",
            MobileImageMounterError::DmgNotFound => "DmgNotFound",
            MobileImageMounterError::SignatureNotFound => "SignatureNotFound",
            MobileImageMounterError::MissingObjectDepenency => "MissingObjectDepenency",
            MobileImageMounterError::UnknownError => "UnknownError",
        })
    }
}

impl From<MobileImageMounterError> for String {
    fn from(value: MobileImageMounterError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum MisagentError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    RequestFailed,
    UnknownError,
}

impl std::error::Error for MisagentError {}

impl From<i32> for MisagentError {
    fn from(value: i32) -> MisagentError {
        match value {
            0 => MisagentError::Success,
            -1 => MisagentError::InvalidArg,
            -2 => MisagentError::PlistError,
            -3 => MisagentError::ConnFailed,
            -4 => MisagentError::RequestFailed,
            _ => MisagentError::UnknownError,
        }
    }
}

impl std::fmt::Display for MisagentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MisagentError::Success => "Success",
            MisagentError::InvalidArg => "InvalidArg",
            MisagentError::PlistError => "PlistError",
            MisagentError::ConnFailed => "ConnFailed",
            MisagentError::RequestFailed => "RequestFailed",
            MisagentError::UnknownError => "UnknownError",
        })
    }
}

impl From<MisagentError> for String {
    fn from(value: MisagentError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum HouseArrestError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    InvalidMode,
    UnknownError,
}

impl std::error::Error for HouseArrestError {}

impl From<i32> for HouseArrestError {
    fn from(value: i32) -> HouseArrestError {
        match value {
            0 => HouseArrestError::Success,
            -1 => HouseArrestError::InvalidArg,
            -2 => HouseArrestError::PlistError,
            -3 => HouseArrestError::ConnFailed,
            -4 => HouseArrestError::InvalidMode,
            _ => HouseArrestError::UnknownError,
        }
    }
}

impl std::fmt::Display for HouseArrestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HouseArrestError::Success => "Success",
            HouseArrestError::InvalidArg => "InvalidArg",
            HouseArrestError::PlistError => "PlistError",
            HouseArrestError::ConnFailed => "ConnFailed",
            HouseArrestError::InvalidMode => "InvalidMode",
            HouseArrestError::UnknownError => "UnknownError",
        })
    }
}

impl From<HouseArrestError> for String {
    fn from(value: HouseArrestError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum HeartbeatError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    UnknownError,
}

impl std::error::Error for HeartbeatError {}

impl From<i32> for HeartbeatError {
    fn from(value: i32) -> HeartbeatError {
        match value {
            0 => HeartbeatError::Success,
            -1 => HeartbeatError::InvalidArg,
            -2 => HeartbeatError::PlistError,
            -3 => HeartbeatError::MuxError,
            -4 => HeartbeatError::SslError,
            -5 => HeartbeatError::NotEnoughData,
            -6 => HeartbeatError::Timeout,
            _ => HeartbeatError::UnknownError,
        }
    }
}

impl std::fmt::Display for HeartbeatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HeartbeatError::Success => "Success",
            HeartbeatError::InvalidArg => "InvalidArg",
            HeartbeatError::PlistError => "PlistError",
            HeartbeatError::MuxError => "MuxError",
            HeartbeatError::SslError => "SslError",
            HeartbeatError::NotEnoughData => "NotEnoughData",
            HeartbeatError::Timeout => "Timeout",
            HeartbeatError::UnknownError => "UnknownError",
        })
    }
}

impl From<HeartbeatError> for String {
    fn from(value: HeartbeatError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum FileRelayError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    InvalidSource,
    StagingEmpty,
    PermissionDenied,
    UnknownError,
}

impl std::error::Error for FileRelayError {}

impl From<i32> for FileRelayError {
    fn from(value: i32) -> FileRelayError {
        match value {
            0 => FileRelayError::Success,
            -1 => FileRelayError::InvalidArg,
            -2 => FileRelayError::PlistError,
            -3 => FileRelayError::MuxError,
            -4 => FileRelayError::InvalidSource,
            -5 => FileRelayError::StagingEmpty,
            -6 => FileRelayError::PermissionDenied,
            _ => FileRelayError::UnknownError,
        }
    }
}

impl std::fmt::Display for FileRelayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FileRelayError::Success => "Success",
            FileRelayError::InvalidArg => "InvalidArg",
            FileRelayError::PlistError => "PlistError",
            FileRelayError::MuxError => "MuxError",
            FileRelayError::InvalidSource => "InvalidSource",
            FileRelayError::StagingEmpty => "StagingEmpty",
            FileRelayError::PermissionDenied => "PermissionDenied",
            FileRelayError::UnknownError => "UnknownError",
        })
    }
}

impl From<FileRelayError> for String {
    fn from(value: FileRelayError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum DiagnosticsRelayError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    UnknownRequest,
    UnknownError,
}

impl std::error::Error for DiagnosticsRelayError {}

impl From<i32> for DiagnosticsRelayError {
    fn from(value: i32) -> DiagnosticsRelayError {
        match value {
            0 => DiagnosticsRelayError::Success,
            -1 => DiagnosticsRelayError::InvalidArg,
            -2 => DiagnosticsRelayError::PlistError,
            -3 => DiagnosticsRelayError::MuxError,
            -4 => DiagnosticsRelayError::UnknownRequest,
            _ => DiagnosticsRelayError::UnknownError,
        }
    }
}

impl std::fmt::Display for DiagnosticsRelayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DiagnosticsRelayError::Success => "Success",
            DiagnosticsRelayError::InvalidArg => "InvalidArg",
            DiagnosticsRelayError::PlistError => "PlistError",
            DiagnosticsRelayError::MuxError => "MuxError",
            DiagnosticsRelayError::UnknownRequest => "UnknownRequest",
            DiagnosticsRelayError::UnknownError => "UnknownError",
        })
    }
}

impl From<DiagnosticsRelayError> for String {
    fn from(value: DiagnosticsRelayError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CompanionProxyError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    OpInProgress,
    NoDevices,
    UnsupportedKey,
    TimeoutReply,
    UnknownError,
}

impl std::error::Error for CompanionProxyError {}

impl From<i32> for CompanionProxyError {
    fn from(value: i32) -> CompanionProxyError {
        match value {
            0 => CompanionProxyError::Success,
            -1 => CompanionProxyError::InvalidArg,
            -2 => CompanionProxyError::PlistError,
            -3 => CompanionProxyError::MuxError,
            -4 => CompanionProxyError::SslError,
            -5 => CompanionProxyError::NotEnoughData,
            -6 => CompanionProxyError::Timeout,
            -7 => CompanionProxyError::OpInProgress,
            -100 => CompanionProxyError::NoDevices,
            -101 => CompanionProxyError::UnsupportedKey,
            -102 => CompanionProxyError::TimeoutReply,
            _ => CompanionProxyError::UnknownError,
        }
    }
}

impl std::fmt::Display for CompanionProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CompanionProxyError::Success => "Success",
            CompanionProxyError::InvalidArg => "InvalidArg",
            CompanionProxyError::PlistError => "PlistError",
            CompanionProxyError::MuxError => "MuxError",
            CompanionProxyError::SslError => "SslError",
            CompanionProxyError::NotEnoughData => "NotEnoughData",
            CompanionProxyError::Timeout => "Timeout",
            CompanionProxyError::OpInProgress => "OpInProgress",
            CompanionProxyError::NoDevices => "NoDevices",
            CompanionProxyError::UnsupportedKey => "UnsupportedKey",
            CompanionProxyError::TimeoutReply => "TimeoutReply",
            CompanionProxyError::UnknownError => "UnknownError",
        })
    }
}

impl From<CompanionProxyError> for String {
    fn from(value: CompanionProxyError) -> String {
        value.to_string()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum AfcError {
    Success,
    UnknownError,
    OpHeaderInvalid,
    NoResources,
    ReadError,
    WriteError,
    UnknownPacketType,
    InvalidArg,
    ObjectNotFound,
    ObjectIsDir,
    PermDenied,
    ServiceNotConnected,
    OpTimeout,
    TooMuchData,
    EndOfData,
    OpNotSupported,
    ObjectExists,
    ObjectBusy,
    NoSpaceLeft,
    OpWouldBlock,
    IoError,
    OpInterrupted,
    OpInProgress,
    InternalError,
    MuxError,
    NoMem,
    NotEnoughData,
    DirNotEmpty,
    ForceSignedType,
}

impl std::error::Error for AfcError {}

impl From<i32> for AfcError {
    fn from(value: i32) -> AfcError {
        match value {
            0 => AfcError::Success,
            1 => AfcError::UnknownError,
            2 => AfcError::OpHeaderInvalid,
            3 => AfcError::NoResources,
            4 => AfcError::ReadError,
            5 => AfcError::WriteError,
            6 => AfcError::UnknownPacketType,
            7 => AfcError::InvalidArg,
            8 => AfcError::ObjectNotFound,
            9 => AfcError::ObjectIsDir,
            10 => AfcError::PermDenied,
            11 => AfcError::ServiceNotConnected,
            12 => AfcError::OpTimeout,
            13 => AfcError::TooMuchData,
            14 => AfcError::EndOfData,
            15 => AfcError::OpNotSupported,
            16 => AfcError::ObjectExists,
            17 => AfcError::ObjectBusy,
            18 => AfcError::NoSpaceLeft,
            19 => AfcError::OpWouldBlock,
            20 => AfcError::IoError,
            21 => AfcError::OpInterrupted,
            22 => AfcError::OpInProgress,
            23 => AfcError::InternalError,
            30 => AfcError::MuxError,
            31 => AfcError::NoMem,
            32 => AfcError::NotEnoughData,
            33 => AfcError::DirNotEmpty,
            _ => AfcError::ForceSignedType,
        }
    }
}

impl std::fmt::Display for AfcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AfcError::Success => "Success",
            AfcError::UnknownError => "UnknownError",
            AfcError::OpHeaderInvalid => "OpHeaderInvalid",
            AfcError::NoResources => "NoResources",
            AfcError::ReadError => "ReadError",
            AfcError::WriteError => "WriteError",
            AfcError::UnknownPacketType => "UnknownPacketType",
            AfcError::InvalidArg => "InvalidArg",
            AfcError::ObjectNotFound => "ObjectNotFound",
            AfcError::ObjectIsDir => "ObjectIsDir",
            AfcError::PermDenied => "PermDenied",
            AfcError::ServiceNotConnected => "ServiceNotConnected",
            AfcError::OpTimeout => "OpTimeout",
            AfcError::TooMuchData => "TooMuchData",
            AfcError::EndOfData => "EndOfData",
            AfcError::OpNotSupported => "OpNotSupported",
            AfcError::ObjectExists => "ObjectExists",
            AfcError::ObjectBusy => "ObjectBusy",
            AfcError::NoSpaceLeft => "NoSpaceLeft",
            AfcError::OpWouldBlock => "OpWouldBlock",
            AfcError::IoError => "IoError",
            AfcError::OpInterrupted => "OpInterrupted",
            AfcError::OpInProgress => "OpInProgress",
            AfcError::InternalError => "InternalError",
            AfcError::MuxError => "MuxError",
            AfcError::NoMem => "NoMem",
            AfcError::NotEnoughData => "NotEnoughData",
            AfcError::DirNotEmpty => "DirNotEmpty",
            AfcError::ForceSignedType => "ForceSignedType",
        })
    }
}

impl From<AfcError> for String {
    fn from(value: AfcError) -> String {
        value.to_string()
    }
}
