use crate::types::{CustomVersion, FGuid};

/// Custom serialization version for changes made in //UE5/Release-* stream
/// See: Engine/Source/Runtime/Core/Public/UObject/UE5ReleaseStreamObjectVersion.h
pub enum EUE5ReleaseStreamObjectVersion {
    BeforeCustomVersionWasAdded = 0,
    ReflectionMethodEnum,
    WorldPartitionActorDescSerializeHLODInfo,
    RemovingTessellation,
    LevelInstanceSerializeRuntimeBehavior,
    PoseAssetRuntimeRefactor,
    WorldPartitionActorDescSerializeActorFolderPath,
    HairStrandsVertexFormatChange,
    AddChaosMaxLinearAngularSpeed,
    PackedLevelInstanceVersion,
    PackedLevelInstanceBoundsFix,
    CustomPropertyAnimGraphNodesUseOptionalPinManager,
    TextFormatArgumentData64bitSupport,
    MaterialLayerStacksAreNotParameters,
    MaterialInterfaceSavedCachedData,
    AddClothMappingLODBias,
    AddLevelActorPackagingScheme,
    WorldPartitionActorDescSerializeAttachParent,
    ConvertedActorGridPlacementToSpatiallyLoadedFlag,
    ActorGridPlacementDeprecateDefaultValueFixup,
    PackedLevelActorUseWorldPartitionActorDesc,
    AddLevelActorFolders,
    RemoveSkeletalMeshLODModelBulkDatas,
    ExcludeBrightnessFromEncodedHDRCubemap,
    VolumetricCloudSampleCountUnification,
    PoseAssetRawDataGUID,
    ConvolutionBloomIntensity,
    WorldPartitionHLODActorDescSerializeHLODSubActors,
    LargeWorldCoordinates,
    AutomaticVersionPlusOne,
}

impl CustomVersion for EUE5ReleaseStreamObjectVersion {
    const GUID: FGuid = FGuid::from_u32(0xD89B5E42, 0x24BD4D46, 0x8412ACA8, 0xDF641779);
}
