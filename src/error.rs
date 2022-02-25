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
    // Internal errors
    MissingObjectDepenency,
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
            -100 => LockdowndError::MissingObjectDepenency,
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
            LockdowndError::MissingObjectDepenency => "MissingObjectDepenency".to_string(),
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
    // Internal 
    MissingObjectDepenency
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
            -100 => InstProxyError::MissingObjectDepenency,
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
            InstProxyError::MissingObjectDepenency => "MissingObjectDependency".to_string(),
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

#[derive(PartialEq, Debug)]
pub enum WebInspectorError {
    Succes,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    NotEnoughData,
    UnknownError
}

impl From<i32> for WebInspectorError {
    fn from(value: i32) -> WebInspectorError {
        match value {
            0 => WebInspectorError::Succes,
            -1 => WebInspectorError::InvalidArg,
            -2 => WebInspectorError::PlistError,
            -3 => WebInspectorError::MuxError,
            -4 => WebInspectorError::SslError,
            -5 => WebInspectorError::ReceiveTimeout,
            -6 => WebInspectorError::NotEnoughData,
            _ => WebInspectorError::UnknownError
        }
    }
}

impl From<WebInspectorError> for String {
    fn from(value: WebInspectorError) -> String {
        match value {
            WebInspectorError::Succes => "Success".to_string(),
            WebInspectorError::InvalidArg => "InvalidArg".to_string(),
            WebInspectorError::PlistError => "PlistError".to_string(),
            WebInspectorError::MuxError => "MuxError".to_string(),
            WebInspectorError::SslError => "SslError".to_string(),
            WebInspectorError::ReceiveTimeout => "ReceiveTimeout".to_string(),
            WebInspectorError::NotEnoughData => "NotEnoughData".to_string(),
            WebInspectorError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SyslogRelayError {
    Success,
    InvalidArg,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    UnknownError
}

impl From<i32> for SyslogRelayError {
    fn from(value: i32) -> SyslogRelayError {
        match value {
            0 => SyslogRelayError::Success,
            -1 => SyslogRelayError::InvalidArg,
            -2 => SyslogRelayError::MuxError,
            -3 => SyslogRelayError::SslError,
            -4 => SyslogRelayError::NotEnoughData,
            -5 => SyslogRelayError::Timeout,
            _ => SyslogRelayError::UnknownError
        }
    }
}

impl From<SyslogRelayError> for String {
    fn from(value: SyslogRelayError) -> String {
        match value {
            SyslogRelayError::Success => "Success".to_string(),
            SyslogRelayError::InvalidArg => "InvalidArg".to_string(),
            SyslogRelayError::MuxError => "MuxError".to_string(),
            SyslogRelayError::SslError => "SslError".to_string(),
            SyslogRelayError::NotEnoughData => "NotEnoughData".to_string(),
            SyslogRelayError::Timeout => "Timeout".to_string(),
            SyslogRelayError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ScreenshotrError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    ReceiveTimeout,
    BadVersion,
    UnknownError
}

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
            _ => ScreenshotrError::UnknownError
        }
    }
}

impl From<ScreenshotrError> for String {
    fn from(value: ScreenshotrError) -> String {
        match value {
            ScreenshotrError::Success => "Success".to_string(),
            ScreenshotrError::InvalidArg => "InvalidArg".to_string(),
            ScreenshotrError::PlistError => "PlistError".to_string(),
            ScreenshotrError::MuxError => "MuxError".to_string(),
            ScreenshotrError::SslError => "SslError".to_string(),
            ScreenshotrError::ReceiveTimeout => "ReceiveTimeout".to_string(),
            ScreenshotrError::BadVersion => "BadVersion".to_string(),
            ScreenshotrError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SbservicesError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    UnknownError,
}

impl From<i32> for SbservicesError {
    fn from(value: i32) -> SbservicesError {
        match value {
            0 => SbservicesError::Success,
            -1 => SbservicesError::InvalidArg,
            -2 => SbservicesError::PlistError,
            -3 => SbservicesError::ConnFailed,
            _ => SbservicesError::UnknownError
        }
    }
}

impl From<SbservicesError> for String {
    fn from(value: SbservicesError) -> String {
        match value {
            SbservicesError::Success => "Success".to_string(),
            SbservicesError::InvalidArg => "InvalidArg".to_string(),
            SbservicesError::PlistError => "PlistError".to_string(),
            SbservicesError::ConnFailed => "ConnFailed".to_string(),
            SbservicesError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ReverseProxyError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    NotEnoughData,
    Timeout,
    UnknownError
}

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
            _ => ReverseProxyError::UnknownError
        }
    }
}

impl From<ReverseProxyError> for String {
    fn from(value: ReverseProxyError) -> String {
        match value {
            ReverseProxyError::Success => "Success".to_string(),
            ReverseProxyError::InvalidArg => "InvalidArg".to_string(),
            ReverseProxyError::PlistError => "PlistError".to_string(),
            ReverseProxyError::MuxError => "MuxError".to_string(),
            ReverseProxyError::SslError => "SslError".to_string(),
            ReverseProxyError::NotEnoughData => "NotEnoughData".to_string(),
            ReverseProxyError::Timeout => "Timeout".to_string(),
            ReverseProxyError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum RestoreError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    NotEnoughData,
    RecieveTimeout,
    UnknownError
}

impl From<i32> for RestoreError {
    fn from(value: i32) -> RestoreError {
        match value {
            0 => RestoreError::Success,
            -1 => RestoreError::InvalidArg,
            -2 => RestoreError::PlistError,
            -3 => RestoreError::MuxError,
            -4 => RestoreError::NotEnoughData,
            -5 => RestoreError::RecieveTimeout,
            _ => RestoreError::UnknownError
        }
    }
}

impl From<RestoreError> for String {
    fn from(value: RestoreError) -> String {
        match value {
            RestoreError::Success => "Success".to_string(),
            RestoreError::InvalidArg => "InvalidArg".to_string(),
            RestoreError::PlistError => "PlistError".to_string(),
            RestoreError::MuxError => "MuxError".to_string(),
            RestoreError::NotEnoughData => "NotEnoughData".to_string(),
            RestoreError::RecieveTimeout => "RecieveTimeout".to_string(),
            RestoreError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => PreboardError::UnknownError
        }
    }
}

impl From<PreboardError> for String {
    fn from(value: PreboardError) -> String {
        match value {
            PreboardError::Success => "Success".to_string(),
            PreboardError::InvalidArg => "InvalidArg".to_string(),
            PreboardError::PlistError => "PlistError".to_string(),
            PreboardError::MuxError => "MuxError".to_string(),
            PreboardError::SslError => "SslError".to_string(),
            PreboardError::NotEnoughData => "NotEnoughData".to_string(),
            PreboardError::Timeout => "Timeout".to_string(),
            PreboardError::OpInProgress => "OpInProgress".to_string(),
            PreboardError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum NpError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    UnknownError,
}

impl From<i32> for NpError {
    fn from(value: i32) -> NpError {
        match value {
            0 => NpError::Success,
            -1 => NpError::InvalidArg,
            -2 => NpError::PlistError,
            -3 => NpError::ConnFailed,
            _ => NpError::UnknownError
        }
    }
}

impl From<NpError> for String {
    fn from(value: NpError) -> String {
        match value {
            NpError::Success => "Success".to_string(),
            NpError::InvalidArg => "InvalidArg".to_string(),
            NpError::PlistError => "PlistError".to_string(),
            NpError::ConnFailed => "ConnFailed".to_string(),
            NpError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => MobileSyncError::UnknownError
        }
    }
}

impl From<MobileSyncError> for String {
    fn from(value: MobileSyncError) -> String {
        match value {
            MobileSyncError::Success => "Success".to_string(),
            MobileSyncError::InvalidArg => "InvalidArg".to_string(),
            MobileSyncError::PlistError => "PlistError".to_string(),
            MobileSyncError::MuxError => "MuxError".to_string(),
            MobileSyncError::SslError => "SslError".to_string(),
            MobileSyncError::ReceiveTimeout => "ReceiveTimeout".to_string(),
            MobileSyncError::BadVersion => "BadVersion".to_string(),
            MobileSyncError::SyncRefused => "SyncRefused".to_string(),
            MobileSyncError::Cancelled => "Cancelled".to_string(),
            MobileSyncError::WrongDirection => "WrongDirection".to_string(),
            MobileSyncError::NotReady => "NotReady".to_string(),
            MobileSyncError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum MobileBackup2Error {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    SslError,
    RecieveTimeout,
    BadVersion,
    ReplyNotOk,
    NoCommonVersion,
    UnknownError,
}

impl From<i32> for MobileBackup2Error {
    fn from(value: i32) -> MobileBackup2Error {
        match value {
            0 => MobileBackup2Error::Success,
            -1 => MobileBackup2Error::InvalidArg,
            -2 => MobileBackup2Error::PlistError,
            -3 => MobileBackup2Error::MuxError,
            -4 => MobileBackup2Error::SslError,
            -5 => MobileBackup2Error::RecieveTimeout,
            -6 => MobileBackup2Error::BadVersion,
            -7 => MobileBackup2Error::ReplyNotOk,
            -8 => MobileBackup2Error::NoCommonVersion,
            _ => MobileBackup2Error::UnknownError
        }
    }
}

impl From<MobileBackup2Error> for String {
    fn from(value: MobileBackup2Error) -> String {
        match value {
            MobileBackup2Error::Success => "Success".to_string(),
            MobileBackup2Error::InvalidArg => "InvalidArg".to_string(),
            MobileBackup2Error::PlistError => "PlistError".to_string(),
            MobileBackup2Error::MuxError => "MuxError".to_string(),
            MobileBackup2Error::SslError => "SslError".to_string(),
            MobileBackup2Error::RecieveTimeout => "RecieveTimeout".to_string(),
            MobileBackup2Error::BadVersion => "BadVersion".to_string(),
            MobileBackup2Error::ReplyNotOk => "ReplyNotOk".to_string(),
            MobileBackup2Error::NoCommonVersion => "NoCommonVersion".to_string(),
            MobileBackup2Error::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => MobileBackupError::UnknownError
        }
    }
}

impl From<MobileBackupError> for String {
    fn from(value: MobileBackupError) -> String {
        match value {
            MobileBackupError::Success => "Success".to_string(),
            MobileBackupError::InvalidArg => "InvalidArg".to_string(),
            MobileBackupError::PlistError => "PlistError".to_string(),
            MobileBackupError::MuxError => "MuxError".to_string(),
            MobileBackupError::SslError => "SslError".to_string(),
            MobileBackupError::ReceiveTimeout => "ReceiveTimeout".to_string(),
            MobileBackupError::BadVersion => "BadVersion".to_string(),
            MobileBackupError::ReplyNotOk => "ReplyNotOk".to_string(),
            MobileBackupError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum MobileActivationError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    UnknownRequest,
    RequestFailed,
    UnknownError,
}

impl From<i32> for MobileActivationError {
    fn from(value: i32) -> MobileActivationError {
        match value {
            0 => MobileActivationError::Success,
            -1 => MobileActivationError::InvalidArg,
            -2 => MobileActivationError::PlistError,
            -3 => MobileActivationError::MuxError,
            -4 => MobileActivationError::UnknownRequest,
            -5 => MobileActivationError::RequestFailed,
            _ => MobileActivationError::UnknownError
        }
    }
}

impl From<MobileActivationError> for String {
    fn from(value: MobileActivationError) -> String {
        match value {
            MobileActivationError::Success => "Success".to_string(),
            MobileActivationError::InvalidArg => "InvalidArg".to_string(),
            MobileActivationError::PlistError => "PlistError".to_string(),
            MobileActivationError::MuxError => "MuxError".to_string(),
            MobileActivationError::UnknownRequest => "UnknownRequest".to_string(),
            MobileActivationError::RequestFailed => "RequestFailed".to_string(),
            MobileActivationError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => MobileImageMounterError::UnknownError
        }
    }
}

impl From<MobileImageMounterError> for String {
    fn from(value: MobileImageMounterError) -> String {
        match value {
            MobileImageMounterError::Success => "Success".to_string(),
            MobileImageMounterError::InvalidArg => "InvalidArg".to_string(),
            MobileImageMounterError::PlistError => "PlistError".to_string(),
            MobileImageMounterError::ConnFailed => "ConnFailed".to_string(),
            MobileImageMounterError::CommandFailed => "CommandFailed".to_string(),
            MobileImageMounterError::DeviceLocked => "DeviceLocked".to_string(),
            MobileImageMounterError::DmgNotFound => "DmgNotFound".to_string(),
            MobileImageMounterError::SignatureNotFound => "SignatureNotFound".to_string(),
            MobileImageMounterError::MissingObjectDepenency => "MissingObjectDepenency".to_string(),
            MobileImageMounterError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum MisagentError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    RequestFailed,
    UnknownError,
}

impl From<i32> for MisagentError {
    fn from(value: i32) -> MisagentError {
        match value {
            0 => MisagentError::Success,
            -1 => MisagentError::InvalidArg,
            -2 => MisagentError::PlistError,
            -3 => MisagentError::ConnFailed,
            -4 => MisagentError::RequestFailed,
            _ => MisagentError::UnknownError
        }
    }
}

impl From<MisagentError> for String {
    fn from(value: MisagentError) -> String {
        match value {
            MisagentError::Success => "Success".to_string(),
            MisagentError::InvalidArg => "InvalidArg".to_string(),
            MisagentError::PlistError => "PlistError".to_string(),
            MisagentError::ConnFailed => "ConnFailed".to_string(),
            MisagentError::RequestFailed => "RequestFailed".to_string(),
            MisagentError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum HouseArrestError {
    Success,
    InvalidArg,
    PlistError,
    ConnFailed,
    InvalidMode,
    UnknownError,
}

impl From<i32> for HouseArrestError {
    fn from(value: i32) -> HouseArrestError {
        match value {
            0 => HouseArrestError::Success,
            -1 => HouseArrestError::InvalidArg,
            -2 => HouseArrestError::PlistError,
            -3 => HouseArrestError::ConnFailed,
            -4 => HouseArrestError::InvalidMode,
            _ => HouseArrestError::UnknownError
        }
    }
}

impl From<HouseArrestError> for String {
    fn from(value: HouseArrestError) -> String {
        match value {
            HouseArrestError::Success => "Success".to_string(),
            HouseArrestError::InvalidArg => "InvalidArg".to_string(),
            HouseArrestError::PlistError => "PlistError".to_string(),
            HouseArrestError::ConnFailed => "ConnFailed".to_string(),
            HouseArrestError::InvalidMode => "InvalidMode".to_string(),
            HouseArrestError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => HeartbeatError::UnknownError
        }
    }
}

impl From<HeartbeatError> for String {
    fn from(value: HeartbeatError) -> String {
        match value {
            HeartbeatError::Success => "Success".to_string(),
            HeartbeatError::InvalidArg => "InvalidArg".to_string(),
            HeartbeatError::PlistError => "PlistError".to_string(),
            HeartbeatError::MuxError => "MuxError".to_string(),
            HeartbeatError::SslError => "SslError".to_string(),
            HeartbeatError::NotEnoughData => "NotEnoughData".to_string(),
            HeartbeatError::Timeout => "Timeout".to_string(),
            HeartbeatError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => FileRelayError::UnknownError
        }
    }
}

impl From<FileRelayError> for String {
    fn from(value: FileRelayError) -> String {
        match value {
            FileRelayError::Success => "Success".to_string(),
            FileRelayError::InvalidArg => "InvalidArg".to_string(),
            FileRelayError::PlistError => "PlistError".to_string(),
            FileRelayError::MuxError => "MuxError".to_string(),
            FileRelayError::InvalidSource => "InvalidSource".to_string(),
            FileRelayError::StagingEmpty => "StagingEmpty".to_string(),
            FileRelayError::PermissionDenied => "PermissionDenied".to_string(),
            FileRelayError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum DiagnosticsRelayError {
    Success,
    InvalidArg,
    PlistError,
    MuxError,
    UnknownRequest,
    UnknownError,
}

impl From<i32> for DiagnosticsRelayError {
    fn from(value: i32) -> DiagnosticsRelayError {
        match value {
            0 => DiagnosticsRelayError::Success,
            -1 => DiagnosticsRelayError::InvalidArg,
            -2 => DiagnosticsRelayError::PlistError,
            -3 => DiagnosticsRelayError::MuxError,
            -4 => DiagnosticsRelayError::UnknownRequest,
            _ => DiagnosticsRelayError::UnknownError
        }
    }
}

impl From<DiagnosticsRelayError> for String {
    fn from(value: DiagnosticsRelayError) -> String {
        match value {
            DiagnosticsRelayError::Success => "Success".to_string(),
            DiagnosticsRelayError::InvalidArg => "InvalidArg".to_string(),
            DiagnosticsRelayError::PlistError => "PlistError".to_string(),
            DiagnosticsRelayError::MuxError => "MuxError".to_string(),
            DiagnosticsRelayError::UnknownRequest => "UnknownRequest".to_string(),
            DiagnosticsRelayError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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
            _ => CompanionProxyError::UnknownError
        }
    }
}

impl From<CompanionProxyError> for String {
    fn from(value: CompanionProxyError) -> String {
        match value {
            CompanionProxyError::Success => "Success".to_string(),
            CompanionProxyError::InvalidArg => "InvalidArg".to_string(),
            CompanionProxyError::PlistError => "PlistError".to_string(),
            CompanionProxyError::MuxError => "MuxError".to_string(),
            CompanionProxyError::SslError => "SslError".to_string(),
            CompanionProxyError::NotEnoughData => "NotEnoughData".to_string(),
            CompanionProxyError::Timeout => "Timeout".to_string(),
            CompanionProxyError::OpInProgress => "OpInProgress".to_string(),
            CompanionProxyError::NoDevices => "NoDevices".to_string(),
            CompanionProxyError::UnsupportedKey => "UnsupportedKey".to_string(),
            CompanionProxyError::TimeoutReply => "TimeoutReply".to_string(),
            CompanionProxyError::UnknownError => "UnknownError".to_string(),
        }
    }
}

#[derive(PartialEq, Debug)]
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

impl From<AfcError> for String {
    fn from(value: AfcError) -> String {
        match value {
            AfcError::Success => "Success".to_string(),
            AfcError::UnknownError => "UnknownError".to_string(),
            AfcError::OpHeaderInvalid => "OpHeaderInvalid".to_string(),
            AfcError::NoResources => "NoResources".to_string(),
            AfcError::ReadError => "ReadError".to_string(),
            AfcError::WriteError => "WriteError".to_string(),
            AfcError::UnknownPacketType => "UnknownPacketType".to_string(),
            AfcError::InvalidArg => "InvalidArg".to_string(),
            AfcError::ObjectNotFound => "ObjectNotFound".to_string(),
            AfcError::ObjectIsDir => "ObjectIsDir".to_string(),
            AfcError::PermDenied => "PermDenied".to_string(),
            AfcError::ServiceNotConnected => "ServiceNotConnected".to_string(),
            AfcError::OpTimeout => "OpTimeout".to_string(),
            AfcError::TooMuchData => "TooMuchData".to_string(),
            AfcError::EndOfData => "EndOfData".to_string(),
            AfcError::OpNotSupported => "OpNotSupported".to_string(),
            AfcError::ObjectExists => "ObjectExists".to_string(),
            AfcError::ObjectBusy => "ObjectBusy".to_string(),
            AfcError::NoSpaceLeft => "NoSpaceLeft".to_string(),
            AfcError::OpWouldBlock => "OpWouldBlock".to_string(),
            AfcError::IoError => "IoError".to_string(),
            AfcError::OpInterrupted => "OpInterrupted".to_string(),
            AfcError::OpInProgress => "OpInProgress".to_string(),
            AfcError::InternalError => "InternalError".to_string(),
            AfcError::MuxError => "MuxError".to_string(),
            AfcError::NoMem => "NoMem".to_string(),
            AfcError::NotEnoughData => "NotEnoughData".to_string(),
            AfcError::DirNotEmpty => "DirNotEmpty".to_string(),
            AfcError::ForceSignedType => "ForceSignedType".to_string(),
        }
    }
}
