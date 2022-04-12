// jkcoxson

/// Transfers files between host and the iDevice
pub mod afc;
/// A proxy for interoping with devices paired with the iOS device
/// This includes the Apple Watch
pub mod companion_proxy;
/// Used for debugging applications on the device
pub mod debug_server;
/// Relays diagnostic logs from the iOS device to the host
pub mod diagnostics_relay;
/// Relays files from the iOS device to the host
pub mod file_relay;
/// A required service for most other services.
/// iOS will close other connections if there is no active heartbeat client
pub mod heartbeat;
/// iTunes file transfer service
pub mod house_arrest;
/// Manages installing, removing and modifying applications on the device
pub mod instproxy;
/// A jumping point for other services
pub mod lockdownd;
/// Manges and checks provisioning profiles
pub mod misagent;
/// Activates the device for iCloud Activation
pub mod mobile_activation;
/// Manages backups of the iOS device's data
/// Contains mobilebackup for < iOS 4 and mobilebackup2 for >= iOS 4
pub mod mobile_backup;
/// A service for mounting developer disk images on devices
pub mod mobile_image_mounter;
/// iTunes mobile sync service
pub mod mobile_sync;
/// Sends and watches for notifications on the iDevice
pub mod notification_proxy;
/// A service that manages data at the first unlock screen after boot.
/// Prepare to be boarded!
pub mod preboard;
/// Usage unknown
pub mod property_list_service;
/// Restores an iDevice to a specific backup or iOS version
pub mod restored;
/// Takes a screenshot and returns it to the host
pub mod screenshotr;
/// Manages the device's OS base and homescreen.
pub mod springboard_services;
/// The iOS device's settings. Very fun to mess with.
pub mod userpref;
/// First used on MacOS, this service is used to inspect the JavaScript and HTML of a site running on the device
pub mod web_inspector;
