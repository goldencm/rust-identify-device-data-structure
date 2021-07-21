//! IDENTIFY_DEVICE_DATA structure
//source: https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ata/ns-ata-_identify_device_data?redirectedfrom=MSDN

struct _IDENTIFY_DEVICE_DATA {
    general_configuration : GeneralConfiguration,

    NumCylinders : u16,
    SpecificConfiguration : u16,
    NumHeads : u16,
    Retired1 : [u16; 2],
    NumSectorsPerTrack : u16,
    VendorUnique1 : [u16; 3],
    SerialNumber : [u8; 20],
    Retired2 : [u16; 2],
    Obsolete1 : u16,
    FirmwareRevision : [u8; 8],
    ModelNumber: [u8; 40],
    MaximumBlockTransfer : u8,
    VendorUnique2 : u8,

    trusted_computing : TrustedComputing,
    
    capabilities : Capabilities,

    ObsoleteWords51 : [u16; 2],
    TranslationFieldsValid : u16,
    Reserved3 : u16,
    FreeFallControlSensitivity : u16,
    NumberOfCurrentCylinders : u16,
    NumberOfCurrentHeads : u16,
    CurrentSectorsPerTrack : u16,
    CurrentSectorCapacity : u32,
    CurrentMultiSectorSetting : u8,
    MultiSectorSettingValid : u8,
    ReservedByte59 : u8,
    SanitizeFeatureSupported : u8,
    CryptoScrambleExtCommandSupported : u8,
    OverwriteExtCommandSupported : u8,
    BlockEraseExtCommandSupported : u8,
    UserAddressableSectors : u32,
    ObsoleteWord62 : u16,
    MultiWordDMASupport : u16,
    MultiWordDMAActive :  u16,
    AdvancedPIOModes :  u16,
    ReservedByte64 :  u16,
    MinimumMWXferCycleTime:  u16,
    RecommendedMWXferCycleTime:  u16,
    MinimumPIOCycleTime:  u16,
    MinimumPIOCycleTimeIORDY:  u16,

    additional_support : AdditionalSupported, 

    ReservedWords70 : [u16; 5],
    QueueDepth : u16,
    ReservedWord75 : u16,

    serial_ata_capabilities : SerialAtaCapabilities,

    serial_ata_features_supported : SerialAtaFeaturesSupported,

    serial_ata_features_enabled : SerialAtaFeaturesEnabled,

    MajorRevision : u16,
    MinorRevision : u16,
    
    command_set_support : CommandSetSupport,

    command_set_active : CommandSetActive,


    UltraDMASupport : u16,
    UltraDMAActive : u16,

    normal_security_erase_unit : NormalSecurityEraseUnit,
    
    enhanced_security_erase_unit : EnhancedSecurityEraseUnit,

    CurrentAPMLevel : u16,
    ReservedWord91 : u16,
    MasterPasswordID : u16,
    HardwareResetResult : u16,
    CurrentAcousticValue : u16,
    RecommendedAcousticValue : u16,
    StreamMinRequestSize : u16,
    StreamingTransferTimeDMA : u16,
    StreamingAccessLatencyDMAPIO : u16,
    uStreamingPerfGranularity : u32,
    Max48BitLBA : [u32; 2],
    StreamingTransferTime : u16,
    DsmCap : u16,

    physical_logical_sector_size : PhysicalLogicalSectorSize,

    InterSeekDelay : u16,
    WorldWideName : [u16; 4],
    ReservedForWorldWideName128 : [u16; 4],
    ReservedForTlcTechnicalReport : u16,
    WordsPerLogicalSector : [u16; 2],

    command_set_support_ext : CommandSetSupportExt,

    command_set_active_ext : CommandSetActiveExt,

    ReservedForExpandedSupportandActive : [u16; 6],
    MsnSupport : u16,
    ReservedWord127 : u16,

    security_status : SecurityStatus,

    ReservedWord129 : [u16; 31],

    cfa_power_mode_1 : CfaPowerMode1,

    ReservedForCfaWord161 : [u16; 7],
    NominalFormFactor : u16,
    ReservedWord168 : u16,

    data_set_management_feature : DataSetManagementFeature,
    
    AdditionalProductID : [u16; 4],
    ReservedForCfaWord174 : [u16; 2],
    CurrentMediaSerialNumber : [u16; 30],

    sct_command_transport : SCTCommandTransport,

    ReservedWord207 : [u16; 2],

    block_alignment : BlockAlignment,
    
    WriteReadVerifySectorCountMode3Only : [u16; 2],
    WriteReadVerifySectorCountMode2Only : [u16; 2],

    nv_cache_capabilities : NVCacheCapabilities,

    NVCacheSizeLSW : u16,
    NVCacheSizeMSW : u16,
    NominalMediaRotationRate : u16,
    ReservedWord218 : u16,

    nv_cache_options : NVCacheOptions,
    
    WriteReadVerifySectorCountMode : u16,
    ReservedWord220 : u16,
    ReservedWord221 : u16,

    transport_major_version : TransportMajorVersion,


    TransportMinorVersion: u16,
    ReservedWord224 : [u16; 6],
    ExtendedNumberOfUserAddressableSectors : [u32; 2],
    MinBlocksPerDownloadMicrocodeMode03 : u16,
    MaxBlocksPerDownloadMicrocodeMode03 : u16,
    ReservedWord236 : [u16; 19],
    Signature : u16,
    CheckSum : u16,
}

struct GeneralConfiguration {
    Reserved1 : u16,
    Retired3 : u16,
    ResponseIncomplete : u16,
    Retired2 : u16,
    FixedDevice : u16,
    RemovableMedia : u16,
    Retired1 : u16,
    DeviceType : u16,
  }

struct TrustedComputing {
    FeatureSupported : u16,
    Reserved : u16,
} 

struct Capabilities{
    CurrentLongPhysicalSectorAlignment : u8,
    ReservedByte49 : u8,
    DmaSupported : u8,
    LbaSupported : u8,
    IordyDisable : u8,
    IordySupported : u8,
    Reserved1 : u8,
    StandybyTimerSupport : u8,
    Reserved2 : u8,
    ReservedWord50 : u16 ,
  }

struct AdditionalSupported {
    ZonedCapabilities : u16,
    NonVolatileWriteCache : u16,
    ExtendedUserAddressableSectorsSupported : u16,
    DeviceEncryptsAllUserData : u16,
    ReadZeroAfterTrimSupported : u16,
    Optional28BitCommandsSupported : u16,
    IEEE1667 : u16,
    DownloadMicrocodeDmaSupported : u16,
    SetMaxSetPasswordUnlockDmaSupported : u16,
    WriteBufferDmaSupported : u16,
    ReadBufferDmaSupported : u16,
    DeviceConfigIdentifySetDmaSupported : u16,
    LPSAERCSupported : u16,
    DeterministicReadAfterTrimSupported : u16,
    CFastSpecSupported : u16,
} 

struct SerialAtaCapabilities {
    Reserved0 : u16,
    SataGenu16 : u16,
    SataGen2 : u16,
    SataGenu16 : u16,
    Reservedu16 : u16,
    NCQ : u16,
    HIPM : u16,
    PhyEvents : u16,
    NcqUnload : u16,
    NcqPriority : u16,
    HostAutoPS : u16,
    DeviceAutoPS : u16,
    ReadLogDMA : u16,
    Reserved2 : u16,
    CurrentSpeed : u16,
    NcqStreaming : u16,
    NcqQueueMgmt : u16,
    NcqReceiveSend : u16,
    DEVSLPtoReducedPwrState : u16,
    Reservedu16 : u16,
}

struct SerialAtaFeaturesSupported{
    Reserved0 : u16,
    NonZeroOffsets : u16,
    DmaSetupAutoActivate : u16,
    DIPM : u16,
    InOrderData : u16,
    HardwareFeatureControl : u16,
    SoftwareSettingsPreservation : u16,
    NCQAutosense : u16,
    DEVSLP : u16,
    HybridInformation : u16,
    Reservedu16 : u16,
}

struct SerialAtaFeaturesEnabled {
    Reserved0 : u16,
    NonZeroOffsets : u16,
    DmaSetupAutoActivate : u16,
    DIPM : u16,
    InOrderData : u16,
    HardwareFeatureControl : u16,
    SoftwareSettingsPreservation : u16,
    DeviceAutoPS : u16,
    DEVSLP : u16,
    HybridInformation : u16,
    Reservedu16 : u16,
} 

struct CommandSetSupport {
    SmartCommands : u16,
    SecurityMode : u16,
    RemovableMediaFeature : u16,
    PowerManagement : u16,
    Reservedu16 : u16,
    WriteCache : u16,
    LookAhead : u16,
    ReleaseInterrupt : u16,
    ServiceInterrupt : u16,
    DeviceReset : u16,
    HostProtectedArea : u16,
    Obsoleteu16 : u16,
    WriteBuffer : u16,
    ReadBuffer : u16,
    Nop : u16,
    Obsolete2 : u16,
    DownloadMicrocode : u16,
    DmaQueued : u16,
    Cfa : u16,
    AdvancedPm : u16,
    Msn : u16,
    PowerUpInStandby : u16,
    ManualPowerUp : u16,
    Reserved2 : u16,
    SetMax : u16,
    Acoustics : u16,
    BigLba : u16,
    DeviceConfigOverlay : u16,
    FlushCache : u16,
    FlushCacheExt : u16,
    WordValid83 : u16,
    SmartErrorLog : u16,
    SmartSelfTest : u16,
    MediaSerialNumber : u16,
    MediaCardPassThrough : u16,
    StreamingFeature : u16,
    GpLogging : u16,
    WriteFua : u16,
    WriteQueuedFua : u16,
    WWN64Bit : u16,
    URGReadStream : u16,
    URGWriteStream : u16,
    ReservedForTechReport : u16,
    IdleWithUnloadFeature : u16,
    WordValid : u16,
} 

struct CommandSetActive {
    SmartCommands : u16,
    SecurityMode : u16,
    RemovableMediaFeature : u16,
    PowerManagement : u16,
    Reserved1 : u16,
    WriteCache : u16,
    LookAhead : u16,
    ReleaseInterrupt : u16,
    ServiceInterrupt : u16,
    DeviceReset : u16,
    HostProtectedArea : u16,
    Obsolete1 : u16,
    WriteBuffer : u16,
    ReadBuffer : u16,
    Nop : u16,
    Obsolete2 : u16,
    DownloadMicrocode : u16,
    DmaQueued : u16,
    Cfa : u16,
    AdvancedPm : u16,
    Msn : u16,
    PowerUpInStandby : u16,
    ManualPowerUp : u16,
    Reserved2 : u16,
    SetMax : u16,
    Acoustics : u16,
    BigLba : u16,
    DeviceConfigOverlay : u16,
    FlushCache : u16,
    FlushCacheExt : u16,
    Resrved3 : u16,
    Words119_120Valid : u16,
    SmartErrorLog : u16,
    SmartSelfTest : u16,
    MediaSerialNumber : u16,
    MediaCardPassThrough : u16,
    StreamingFeature : u16,
    GpLogging : u16,
    WriteFua : u16,
    WriteQueuedFua : u16,
    WWN64Bit : u16,
    URGReadStream : u16,
    URGWriteStream : u16,
    ReservedForTechReport : u16,
    IdleWithUnloadFeature : u16,
    Reserved4 : u16,
}

struct NormalSecurityEraseUnit {
    TimeRequired : u16,
    ExtendedTimeReported : u16,
}

struct EnhancedSecurityEraseUnit {
    TimeRequired : u16,
    ExtendedTimeReported : u16,
}

struct PhysicalLogicalSectorSize {
    LogicalSectorsPerPhysicalSector : u16,
    Reserved0 : u16,
    LogicalSectorLongerThan256Words : u16,
    MultipleLogicalSectorsPerPhysicalSector : u16,
    Reserved1 : u16,
} 

struct CommandSetSupportExt {
    ReservedForDrqTechnicalReport : u16,
    WriteReadVerify : u16,
    WriteUncorrectableExt : u16,
    ReadWriteLogDmaExt : u16,
    DownloadMicrocodeMode3 : u16,
    FreefallControl : u16,
    SenseDataReporting : u16,
    ExtendedPowerConditions : u16,
    Reserved0 : u16,
    WordValid : u16,
} 

struct CommandSetActiveExt{
    ReservedForDrqTechnicalReport : u16,
    WriteReadVerify : u16,
    WriteUncorrectableExt : u16,
    ReadWriteLogDmaExt : u16,
    DownloadMicrocodeMode3 : u16,
    FreefallControl : u16,
    SenseDataReporting : u16,
    ExtendedPowerConditions : u16,
    Reserved0 : u16,
    Reserved1 : u16,
} 

struct SecurityStatus {
    SecuritySupported : u16,
    SecurityEnabled : u16,
    SecurityLocked : u16,
    SecurityFrozen : u16,
    SecurityCountExpired : u16,
    EnhancedSecurityEraseSupported : u16,
    Reserved0 : u16,
    SecurityLevel : u16,
    Reserved1 : u16,
} 

struct CfaPowerMode1 {
    MaximumCurrentInMA : u16,
    CfaPowerMode1Disabled : u16,
    CfaPowerMode1Required : u16,
    Reserved0 : u16,
    Word160Supported : u16,
} 

struct DataSetManagementFeature{
    SupportsTrim : u16,
    Reserved0 : u16,
}

struct SCTCommandTransport {
    Supported : u16,
    Reserved0 : u16,
    WriteSameSuported : u16,
    ErrorRecoveryControlSupported : u16,
    FeatureControlSuported : u16,
    DataTablesSuported : u16,
    Reserved1 : u16,
    VendorSpecific : u16,
} 

struct BlockAlignment {
    AlignmentOfLogicalWithinPhysical : u16,
    Word209Supported : u16,
    Reserved0 : u16,
} 

struct NVCacheCapabilities {
    NVCachePowerModeEnabled : u16,
    Reserved0 : u16,
    NVCacheFeatureSetEnabled : u16,
    Reserved1 : u16,
    NVCachePowerModeVersion : u16,
    NVCacheFeatureSetVersion : u16,
} 

struct NVCacheOptions {
    NVCacheEstimatedTimeToSpinUpInSeconds : u8,
    Reserved : u8,
} 

struct TransportMajorVersion{
    MajorVersion : u16,
    TransportType : u16,
}

