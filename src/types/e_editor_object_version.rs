use crate::types::{CustomVersion, FGuid};

/// Custom serialization version for changes made in Dev-Editor stream.
/// See: Engine/Source/Runtime/Core/Public/UObject/EditorObjectVersion.h
pub enum EEditorObjectVersion {
    BeforeCustomVersionWasAdded = 0,
    GatheredTextProcessVersionFlagging,
    GatheredTextPackageCacheFixesV1,
    RootMetaDataSupport,
    GatheredTextPackageCacheFixesV2,
    TextFormatArgumentDataIsVariant,
    SplineComponentCurvesInStruct,
    ComboBoxControllerSupportUpdate,
    RefactorMeshEditorMaterials,
    AddedFontFaceAssets,
    UPropertryForMeshSection,
    WidgetGraphSchema,
    AddedBackgroundBlurContentSlot,
    StableUserDefinedEnumDisplayNames,
    AddedInlineFontFaceAssets,
    UPropertryForMeshSectionSerialize,
    FastWidgetTemplates,
    MaterialThumbnailRenderingChanges,
    NewSlateClippingSystem,
    MovieSceneMetaDataSerialization,
    GatheredTextEditorOnlyPackageLocId,
    AddedAlwaysSignNumberFormattingOption,
    AddedMaterialSharedInputs,
    AddedMorphTargetSectionIndices,
    SerializeInstancedStaticMeshRenderData,
    MeshDescriptionNewSerializationMovedToRelease,
    MeshDescriptionNewAttributeFormat,
    ChangeSceneCaptureRootComponent,
    StaticMeshDeprecatedRawMesh,
    MeshDescriptionBulkDataGuid,
    MeshDescriptionRemovedHoles,
    ChangedWidgetComponentWindowVisibilityDefault,
    CultureInvariantTextSerializationKeyStability,
    ScrollBarThicknessChange,
    RemoveLandscapeHoleMaterial,
    MeshDescriptionTriangles,
    ComputeWeightedNormals,
    SkeletalMeshBuildRefactor,
    SkeletalMeshMoveEditorSourceDataToPrivateAsset,
    NumberParsingOptionsNumberLimitsAndClamping,
    SkeletalMeshSourceDataSupport16bitOfMaterialNumber,
}

impl CustomVersion for EEditorObjectVersion {
    const GUID: FGuid = FGuid::from_u32(0xE4B068ED, 0xF49442E9, 0xA231DA0B, 0x2E46BB41);
}
