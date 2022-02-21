// jkcoxson
// Enums for libimobiledevice's error codes

#[derive(PartialEq, Debug)]
pub enum LockdownError {
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

impl From<i32> for LockdownError {
    fn from(i: i32) -> LockdownError {
        match i {
            0 => LockdownError::Success,
            -1 => LockdownError::InvalidArg,
            -2 => LockdownError::InvalidConf,
            -3 => LockdownError::PlistError,
            -4 => LockdownError::PairingFailed,
            -5 => LockdownError::SslError,
            -6 => LockdownError::DictError,
            -7 => LockdownError::RecieveTimeout,
            -8 => LockdownError::MuxError,
            -9 => LockdownError::NoRunningSession,
            -10 => LockdownError::InvalidResponse,
            -11 => LockdownError::MissingKey,
            -12 => LockdownError::MissingValue,
            -13 => LockdownError::GetProhibited,
            -14 => LockdownError::SetProhibited,
            -15 => LockdownError::RemoveProhibited,
            -16 => LockdownError::ImmutableValue,
            -17 => LockdownError::PasswordProtected,
            -18 => LockdownError::UserDeniedPairing,
            -19 => LockdownError::PairingDialogueRepsonsePending,
            -20 => LockdownError::MissingHostId,
            -21 => LockdownError::InvalidHostId,
            -22 => LockdownError::SessionActive,
            -23 => LockdownError::SessionInactive,
            -24 => LockdownError::MissingSessionId,
            -25 => LockdownError::InvalidSessionId,
            -26 => LockdownError::MissingService,
            -27 => LockdownError::InvalidService,
            -28 => LockdownError::ServiceLimit,
            -29 => LockdownError::MissingPairRecord,
            -30 => LockdownError::SavePairRecordFailed,
            -31 => LockdownError::InvalidPairRecord,
            -32 => LockdownError::InvalidActivationRecord,
            -33 => LockdownError::MissingActivationRecord,
            -34 => LockdownError::ServiceProhibited,
            -35 => LockdownError::EscrowLocked,
            -36 => LockdownError::PairingProhibitedOverThisConnection,
            -37 => LockdownError::FmipProtected,
            -38 => LockdownError::McProtected,
            -39 => LockdownError::McChallengeRequired,
            _ => LockdownError::UnknownError,
        }
    }
}

impl From<LockdownError> for String {
    fn from(e: LockdownError) -> String {
        match e {
            LockdownError::Success => "Success".to_string(),
            LockdownError::InvalidArg => "InvalidArg".to_string(),
            LockdownError::InvalidConf => "InvalidConf".to_string(),
            LockdownError::PlistError => "PlistError".to_string(),
            LockdownError::PairingFailed => "PairingFailed".to_string(),
            LockdownError::SslError => "SslError".to_string(),
            LockdownError::DictError => "DictError".to_string(),
            LockdownError::RecieveTimeout => "RecieveTimeout".to_string(),
            LockdownError::MuxError => "MuxError".to_string(),
            LockdownError::NoRunningSession => "NoRunningSession".to_string(),
            LockdownError::InvalidResponse => "InvalidResponse".to_string(),
            LockdownError::MissingKey => "MissingKey".to_string(),
            LockdownError::MissingValue => "MissingValue".to_string(),
            LockdownError::GetProhibited => "GetProhibited".to_string(),
            LockdownError::SetProhibited => "SetProhibited".to_string(),
            LockdownError::RemoveProhibited => "RemoveProhibited".to_string(),
            LockdownError::ImmutableValue => "ImmutableValue".to_string(),
            LockdownError::PasswordProtected => "PasswordProtected".to_string(),
            LockdownError::UserDeniedPairing => "UserDeniedPairing".to_string(),
            LockdownError::PairingDialogueRepsonsePending => {
                "PairingDialogueRepsonsePending".to_string()
            }
            LockdownError::MissingHostId => "MissingHostId".to_string(),
            LockdownError::InvalidHostId => "InvalidHostId".to_string(),
            LockdownError::SessionActive => "SessionActive".to_string(),
            LockdownError::SessionInactive => "SessionInactive".to_string(),
            LockdownError::MissingSessionId => "MissingSessionId".to_string(),
            LockdownError::InvalidSessionId => "InvalidSessionId".to_string(),
            LockdownError::MissingService => "MissingService".to_string(),
            LockdownError::InvalidService => "InvalidService".to_string(),
            LockdownError::ServiceLimit => "ServiceLimit".to_string(),
            LockdownError::MissingPairRecord => "MissingPairRecord".to_string(),
            LockdownError::SavePairRecordFailed => "SavePairRecordFailed".to_string(),
            LockdownError::InvalidPairRecord => "InvalidPairRecord".to_string(),
            LockdownError::InvalidActivationRecord => "InvalidActivationRecord".to_string(),
            LockdownError::MissingActivationRecord => "MissingActivationRecord".to_string(),
            LockdownError::ServiceProhibited => "ServiceProhibited".to_string(),
            LockdownError::EscrowLocked => "EscrowLocked".to_string(),
            LockdownError::PairingProhibitedOverThisConnection => {
                "PairingProhibitedOverThisConnection".to_string()
            }
            LockdownError::FmipProtected => "FmipProtected".to_string(),
            LockdownError::McProtected => "McProtected".to_string(),
            LockdownError::McChallengeRequired => "McChallengeRequired".to_string(),
            LockdownError::UnknownError => "UnknownError".to_string(),
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
