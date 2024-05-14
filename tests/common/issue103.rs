use std::collections::HashMap;

const KEY_1: &str = "LandscapeData.MapProperty.Value.StructProperty";
const KEY_2: &str = "Buildings.ArrayProperty.ToolsBuffer_118_137A92324AB6747A399BA4B732BE0166.MapProperty.Value.StructProperty";
const KEY_3: &str = "NPCs.ArrayProperty.PresetsIDs_166_980BA472412B10BA3984DCB92C30E02A.MapProperty.Value.StructProperty";

pub(crate) fn hints() -> HashMap<String, String> {
    HashMap::from([
        (KEY_1.to_string(), "Unknown".to_string()),
        (KEY_2.to_string(), "Unknown".to_string()),
        // (KEY_3.to_string(), "???".to_string()),
    ])
}
