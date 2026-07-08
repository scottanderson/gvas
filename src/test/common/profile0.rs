use std::collections::HashMap;

pub(crate) fn hints() -> HashMap<String, String> {
    HashMap::from([(
        "achievementHistoryScope.StructProperty.metrics.MapProperty.Value.StructProperty"
            .to_string(),
        "this type hint is unused".to_string(),
    )])
}

pub(crate) const PROFILE_0_JSON: &str = r#"{
  "header": {
    "file_version": 522,
    "engine_version": {
      "major": 4,
      "minor": 27,
      "patch": 2,
      "change_list": 0,
      "branch": "++UE4+Release-4.27"
    },
    "custom_versions": {
      "fb26e412-1f15-4b4d-9372-550a961d2f70": 3,
      "24bb7af3-5646-4f83-1f2f-2dc249ad96ff": 5,
      "fcf57afa-5076-4283-b9a9-e658ffa02d32": 68,
      "11310aed-2e55-4d61-af67-9aa3c5a1082c": 17,
      "76a52329-0923-45b5-98ae-d841cf2f6ad8": 5,
      "5fbc6907-55c8-40ae-8e67-f1845efff13f": 1,
      "82e77c4e-3323-43a5-b46b-13c597310df3": 0,
      "9c54d522-a826-4fbe-9421-074661b482d0": 43,
      "b0d832e4-1f89-4f0d-accf-7eb736fd4aa2": 10,
      "e1c64328-a22c-4d53-a36c-8e866417bd8c": 0,
      "375ec13c-06e4-48fb-b500-84f0262a717e": 4,
      "e4b068ed-f494-42e9-a231-da0b2e46bb41": 40,
      "cffc743f-43b0-4480-9391-14df171d2073": 37,
      "b02b49b5-bb20-44e9-a304-32b752e40360": 3,
      "a4e4105c-59a1-49b5-a7c5-40c4547edfee": 0,
      "39c831c9-5ae6-47dc-9a44-9c173e1c8e7c": 0,
      "78f01b33-ebea-4f98-b9b4-84eaccb95aa2": 14,
      "6631380f-2d4d-43e0-8009-cf276956a95a": 0,
      "12f88b9f-8875-4afc-a67c-d90c383abd29": 45,
      "7b5ae74c-d270-4c10-a958-57980b212a5a": 13,
      "d7296918-1dd6-4bdd-9de2-64a83cc13884": 3,
      "c2a15278-bfe7-4afe-6c17-90ff531df755": 1,
      "6eaca3d4-40ec-4cc1-b786-8bed09428fc5": 3,
      "29e575dd-e0a3-4627-9d10-d276232cdcea": 17,
      "af43a65d-7fd3-4947-9873-3e8ed9c1bb05": 15,
      "6b266cec-1ec7-4b8f-a30b-e4d90942fc07": 1,
      "0df73d61-a23f-47ea-b727-89e90c41499a": 1,
      "601d1886-ac64-4f84-aa16-d3de0deac7d6": 47,
      "e7086368-6b23-4c58-8439-1b7016265e91": 1,
      "9dffbcd6-494f-0158-e221-12823c92a888": 10,
      "f2aed0ac-9afe-416f-8664-aa7ffa26d6fc": 1,
      "174f1f0b-b4c6-45a5-b13f-2ee8d0fb917d": 10,
      "35f94a83-e258-406c-a318-09f59610247c": 41,
      "b68fc16e-8b1b-42e2-b453-215c058844fe": 1,
      "b2e18506-4273-cfc2-a54e-f4bb758bba07": 1,
      "64f58936-fd1b-42ba-ba96-7289d5d0fa4e": 1,
      "6f0ed827-a609-4895-9c91-998d90180ea4": 2,
      "717f9ee7-e9b0-493a-88b3-91321b388107": 8,
      "54683250-8099-48af-8bc8-9896fbadf9b7": 0,
      "430c4d19-7154-4970-8769-9b69df90b0e5": 15,
      "aafe32bd-5395-4c14-b66a-5e251032d1dd": 1,
      "23afe18e-4ce1-4e58-8d61-c252b953beb7": 11,
      "a462b7ea-f499-4e3a-99c1-ec1f8224e1b2": 4,
      "2eb5fdbd-01ac-4d10-8136-f38f3393a5da": 5,
      "509d354f-f6e6-492f-a749-85b2073c631c": 0,
      "4a56eb40-10f5-11dc-92d3-347eb2c96ae7": 2,
      "d78a4a00-e858-4697-baa8-19b5487d46b4": 18,
      "5579f886-933a-4c1f-83ba-087b6361b92f": 2,
      "612fbe52-da53-400b-910d-4f919fb1857c": 1,
      "a4237a36-caea-41c9-8fa2-18f858681bf3": 4,
      "804e3f75-7088-4b49-a4d6-8c063c7eb6dc": 5,
      "fb680af2-59ef-4ba3-baa8-19b573c8443d": 2,
      "9950b70e-b41a-4e17-bbcc-fa0d57817fd6": 1
    },
    "save_game_class_name": "/Script/EconCore.GenericSaveGame"
  },
  "properties": {
    "dbStrings": {
      "type": "MapProperty",
      "str_strs": {
        "Story/Campaign/index": "c1.m19a.s2",
        "music": "music.lasthope",
        "preferedEngine": "DieselEngine",
        "Option_settings.a.master": "50.0%",
        "Option_settings.a.music": "50.0%",
        "Option_settings.g.terrainres": "tfactor.x2",
        "Option_settings.dpiscale": "120%",
        "Option_settings.daytime": "toggle.enabled",
        "preferedTrainColor": "Crimson"
      }
    },
    "dbNumbers": {
      "type": "MapProperty",
      "str_ints": {
        "SeenMovie_Welcome": 1,
        "meta.attemptingload": 0,
        "appearin_lastsfx_time_intro": 8734,
        "appearin_lastsfx_time_outro": 8738,
        "playtime_418AD60A45FCE310E9D3DEB5B9894EF2": 38,
        "bestrank_418AD60A45FCE310E9D3DEB5B9894EF2": 4,
        "money": 14,
        "roamPlayerPosition_x": -1366,
        "roamPlayerPosition_y": -934,
        "playtime_2AB6B5764521494B8DE77891EFD0A8AE": 68,
        "bestrank_2AB6B5764521494B8DE77891EFD0A8AE": 4,
        "playtime_9696568941466D44AE95DD9F1F170004": 67,
        "bestrank_9696568941466D44AE95DD9F1F170004": 4,
        "playstyle.modifier.noghosting": 0,
        "playstyle.modifier.notimer": 0,
        "playstyle.modifier.infinitemoney": 0,
        "playstyle.modifier.endless": 0,
        "playtime_197C3AD14915E04F243623981F89CA32": 496,
        "bestrank_197C3AD14915E04F243623981F89CA32": 4,
        "didNotify_unlock.placeable.oilwell": 1,
        "didNotify_unlock.upgrade.oilpower": 1,
        "playtime_275848C64EC0DFAC00DF819482F5E31D": 802,
        "bestrank_275848C64EC0DFAC00DF819482F5E31D": 4,
        "didNotify_unlock.BasicEngine": 1,
        "playtime_82A9B46F42BF6C5555D608A6619B35E6": 1374,
        "bestrank_82A9B46F42BF6C5555D608A6619B35E6": 4,
        "didNotify_unlock.CustomEngine": 1,
        "didNotify_unlock.placeable.geothermal": 1,
        "playtime_976B945848D92B929BC9318144D2AFD9": 1615,
        "bestrank_976B945848D92B929BC9318144D2AFD9": 2,
        "didNotify_unlock.placeable.oilpower": 1,
        "didNotify_unlock.placeable.limestonemine": 1,
        "didNotify_unlock.placeable.coppermine": 1,
        "didNotify_unlock.placeable.concreteyard": 1,
        "didNotify_unlock.upgrade.concreteyard": 1,
        "didNotify_unlock.upgrade.coiler": 1,
        "playtime_23565CB1440C2F8CF29A20BE0F344E14": 1761,
        "bestrank_23565CB1440C2F8CF29A20BE0F344E14": 3,
        "didNotify_unlock.autoBranching": 1,
        "playtime_C40ABA944C06215E9BA1F3808798BD78": 866,
        "bestrank_C40ABA944C06215E9BA1F3808798BD78": 4,
        "playtime_ABD6676B4D2F295C2C56F7BC7A80CF1B": 1049,
        "bestrank_ABD6676B4D2F295C2C56F7BC7A80CF1B": 4,
        "didNotify_unlock.heightIncrease1": 1,
        "playtime_0A465B724263583BBBAC469FE7D852F8": 2255,
        "bestrank_0A465B724263583BBBAC469FE7D852F8": 1,
        "didNotify_unlock.placeable.saltsifter": 1,
        "didNotify_unlock.placeable.coalmine": 1,
        "didNotify_unlock.upgrade.saltsifter": 1,
        "playtime_7CA6B81C4B27D3DD5D9110A91BD3B333": 3046,
        "bestrank_7CA6B81C4B27D3DD5D9110A91BD3B333": 1,
        "didNotify_unlock.placeable.coalpower": 1,
        "didNotify_unlock.ClimberEngine": 1,
        "didNotify_unlock.placeable.ironmine": 1,
        "playtime_E2AB4F1B4783E580DF19F5BB1048C40C": 1258,
        "bestrank_E2AB4F1B4783E580DF19F5BB1048C40C": 4,
        "didNotify_unlock.placeable.steelmill": 1,
        "playtime_E54ECA4B4EBE680952C1E1A108EBFED6": 1402,
        "bestrank_E54ECA4B4EBE680952C1E1A108EBFED6": 4,
        "didNotify_unlock.placeable.plasticizer": 1,
        "playtime_3D2D8F324CA52B457413909B8FA06DBD": 3128,
        "bestrank_3D2D8F324CA52B457413909B8FA06DBD": 2,
        "didNotify_unlock.placeable.furnitureAssembler": 1,
        "didNotify_unlock.upgrade.plasticizer": 1,
        "didNotify_unlock.upgrade.furnitureassembler": 1,
        "didNotify_unlock.upgrade.coalpower": 1,
        "didNotify_unlock.upgrade.steelmill": 1,
        "playtime_79C004794149BAEF9D01D0812D468C73": 1699,
        "bestrank_79C004794149BAEF9D01D0812D468C73": 3,
        "playtime_C351F593458BA314057C44964D17796E": 1908,
        "bestrank_C351F593458BA314057C44964D17796E": 4,
        "didNotify_unlock.placeable.goodsfactory": 1,
        "didNotify_unlock.placeable.oilrefinery": 1,
        "didNotify_unlock.placeable.hardmold": 1,
        "didNotify_unlock.placeable.neonrefinery": 1,
        "playtime_CE7047994E27F2501309BFBF10504939": 2497,
        "bestrank_CE7047994E27F2501309BFBF10504939": 3,
        "playtime_6F26D84E49EF7EE051658D88685804A3": 2789,
        "bestrank_6F26D84E49EF7EE051658D88685804A3": 2,
        "didNotify_unlock.placeable.incinerator": 1,
        "didNotify_unlock.placeable.oregassifier": 1,
        "didNotify_unlock.placeable.electronicsfab": 1,
        "didNotify_unlock.upgrade.goodsfactory": 1,
        "playtime_7792A16A471FD6ABABB19C909A879023": 4257,
        "bestrank_7792A16A471FD6ABABB19C909A879023": 2,
        "playtime_A8C916CA44A8F236048AC88B3B9D9D1B": 279,
        "bestrank_A8C916CA44A8F236048AC88B3B9D9D1B": 4,
        "playtime_8017CDB54D7D1D368B309198831A15F1": 2814,
        "bestrank_8017CDB54D7D1D368B309198831A15F1": 4,
        "didNotify_unlock.placeable.tooldie": 1,
        "didNotify_unlock.upgrade.tooldie": 1,
        "playtime_2E33E5724251952E750D6FA29B6A6242": 3373,
        "bestrank_2E33E5724251952E750D6FA29B6A6242": 3,
        "playtime_BDE017E34B75C14E86D9AA973EFE3385": 1571,
        "bestrank_BDE017E34B75C14E86D9AA973EFE3385": 4,
        "didNotify_unlock.upgrade.oilrefinery": 1,
        "playtime_FDE6EA5B41E9DF7229789A93AE4E8170": 2556,
        "bestrank_FDE6EA5B41E9DF7229789A93AE4E8170": 3,
        "playtime_E6D6A92C43B137E2C8B402A0F43E5F51": 2366,
        "bestrank_E6D6A92C43B137E2C8B402A0F43E5F51": 4,
        "didNotify_unlock.placeable.hullyard": 1,
        "didNotify_unlock.heightIncrease2": 1,
        "didNotify_unlock.HybridEngine": 1,
        "didNotify_unlock.upgrade.neonrefinery": 1,
        "didNotify_unlock.upgrade.electronicsfab": 1,
        "didNotify_unlock.upgrade.hullyard": 1,
        "didNotify_unlock.upgrade.incinerator": 1,
        "didNotify_unlock.upgrade.oregassifier": 1,
        "didNotify_unlock.upgrade.hardmold": 1,
        "playtime_49106FD1406C726B44788684DD19013B": 3678,
        "bestrank_49106FD1406C726B44788684DD19013B": 3,
        "didNotify_unlock.placeable.luxuryassembler": 1,
        "playtime_19FC4F6F4F082AEF91001DB636DD3CB0": 2258,
        "bestrank_19FC4F6F4F082AEF91001DB636DD3CB0": 4,
        "didNotify_unlock.placeable.glasssmelter": 1,
        "playtime_B9C9338A41EB24590F4F3D8F77E1DD59": 5376,
        "bestrank_B9C9338A41EB24590F4F3D8F77E1DD59": 4,
        "didNotify_unlock.placeable.signworks": 1,
        "didNotify_unlock.placeable.chipfab": 1,
        "playtime_1F2DF7A54B9EF8A02928BFAF03675B92": 1187,
        "bestrank_1F2DF7A54B9EF8A02928BFAF03675B92": 4,
        "didNotify_unlock.upgrade.glasssmelter": 1,
        "didNotify_unlock.stationSpeed": 1,
        "playtime_F65B1395431DE131107D1BA87B09A133": 6844,
        "bestrank_F65B1395431DE131107D1BA87B09A133": 2,
        "didNotify_unlock.placeable.tubeplant": 1,
        "didNotify_unlock.placeable.motorAssembly": 1,
        "didNotify_unlock.upgrade.motorAssembly": 1,
        "playtime_AB6DD6AB4678ACAC44968D8ABDD312E3": 6083,
        "bestrank_AB6DD6AB4678ACAC44968D8ABDD312E3": 3,
        "didNotify_unlock.placeable.heavyworks": 1,
        "didNotify_unlock.placeable.framer": 1,
        "playtime_6071814D4CF0425177A7F39AC189358B": 1383,
        "bestrank_6071814D4CF0425177A7F39AC189358B": 4,
        "playtime_85EB724D43C96BAA7832A3BAA3AEDEC2": 712,
        "bestrank_85EB724D43C96BAA7832A3BAA3AEDEC2": 4,
        "playtime_7CA27AEE46CC848BE165FEA4F8081A9C": 1167,
        "bestrank_7CA27AEE46CC848BE165FEA4F8081A9C": 4,
        "playtime_02700E05478FA5C176BA6F9F97675886": 1310,
        "bestrank_02700E05478FA5C176BA6F9F97675886": 4,
        "playtime_428959F042082EEB444E56AC0D976004": 1366,
        "bestrank_428959F042082EEB444E56AC0D976004": 4,
        "playtime_8633BE2044A6EC9315E5F6BAA128ACE8": 1273,
        "bestrank_8633BE2044A6EC9315E5F6BAA128ACE8": 4,
        "didNotify_unlock.DieselEngine": 1,
        "playtime_573832E84B1534B065BF6693C7523FC4": 1832,
        "bestrank_573832E84B1534B065BF6693C7523FC4": 4,
        "playtime_977D0299439D9BF70DFC5BB4CF27591C": 1648,
        "bestrank_977D0299439D9BF70DFC5BB4CF27591C": 4,
        "playtime_CCEE486545CECC02A0810280F994B0D8": 3575,
        "bestrank_CCEE486545CECC02A0810280F994B0D8": 2,
        "playtime_030D7E264B5C212715FEFE80F6EB97E2": 3677,
        "bestrank_030D7E264B5C212715FEFE80F6EB97E2": 3,
        "didNotify_unlock.upgrade.signworks": 1,
        "playtime_A92DE3C648A91DFDF02B598AE39A3DCA": 4637,
        "bestrank_A92DE3C648A91DFDF02B598AE39A3DCA": 4,
        "didNotify_unlock.upgrade.chipfab": 1,
        "playtime_11EC647042ECCC315E1B909C4D4A9666": 2706,
        "bestrank_11EC647042ECCC315E1B909C4D4A9666": 4,
        "didNotify_unlock.RescueEngine": 1,
        "didNotify_unlock.upgrade.tubeplant": 1,
        "didNotify_unlock.upgrade.luxuryassembler": 1,
        "didNotify_unlock.upgrade.framer": 1,
        "didNotify_unlock.upgrade.heavyworks": 1,
        "playtime_173683C94F06A5F7B0D24F80BA2CF073": 4455,
        "bestrank_173683C94F06A5F7B0D24F80BA2CF073": 3,
        "playtime_E312638F4BED14780F466B95B370E0DD": 2011,
        "bestrank_E312638F4BED14780F466B95B370E0DD": 4,
        "playtime_E23150FF41457A230C7FC4B84226C0C5": 5053,
        "bestrank_E23150FF41457A230C7FC4B84226C0C5": 4,
        "playtime_4D163CA741A2EDAA7ED54C83D77539FF": 5906,
        "bestrank_4D163CA741A2EDAA7ED54C83D77539FF": 3,
        "playtime_A698A0C242D5A317F8DB6D97EF0AFF8E": 4176,
        "bestrank_A698A0C242D5A317F8DB6D97EF0AFF8E": 4,
        "didNotify_unlock.placeable.rocketfactory": 1,
        "didNotify_unlock.placeable.boosterplant": 1,
        "didNotify_unlock.placeable.armory": 1,
        "didNotify_unlock.BulletEngine": 1,
        "didNotify_unlock.upgrade.rocketfactory": 1,
        "playtime_70B1F2D8471C128390408C94C7EB161E": 1914,
        "bestrank_70B1F2D8471C128390408C94C7EB161E": 4
      }
    },
    "UnlockLayer": {
      "type": "StructProperty",
      "type_name": "UnlockLayer",
      "CustomStruct": {
        "ownCounts": [
          {
            "type": "MapProperty",
            "name_ints": {
              "unlock.c1.t2": 1,
              "unlock.c1.t3": 1,
              "unlock.c1.m1": 1,
              "unlock.c1.m2": 1,
              "unlock.milestone.oilwell": 1,
              "unlock.upgrade.oilpower": 1,
              "unlock.milestone.Workhorse": 1,
              "unlock.c1.m2bonus": 1,
              "unlock.c1.m3": 1,
              "unlock.CustomEngine": 1,
              "unlock.placeable.geothermal": 1,
              "unlock.upgrade.geothermal": 1,
              "unlock.milestone.oilPower": 1,
              "unlock.c1.m4": 1,
              "unlock.milestone.coppermine": 1,
              "unlock.milestone.coiler": 1,
              "unlock.milestone.limestonemine": 1,
              "unlock.milestone.concreteYard": 1,
              "unlock.upgrade.concreteyard": 1,
              "unlock.upgrade.coiler": 1,
              "unlock.c1.mspecial1": 1,
              "unlock.milestone.autoBranching": 1,
              "unlock.c1.m6": 1,
              "unlock.milestone.MediumHeight": 1,
              "unlock.c1.m7": 1,
              "unlock.milestone.saltsifter": 1,
              "unlock.milestone.coalMine": 1,
              "unlock.upgrade.saltsifter": 1,
              "unlock.c1.m8": 1,
              "unlock.milestone.coalpower": 1,
              "unlock.milestone.Industrial": 1,
              "unlock.milestone.ironmine": 1,
              "unlock.c1.m9": 1,
              "unlock.c1.m8bonus1": 1,
              "unlock.c1.m8bonus2": 1,
              "unlock.milestone.steelMill": 1,
              "unlock.milestone.plasticizer": 1,
              "unlock.c1.mspecial2": 1,
              "unlock.c1.m9bonus": 1,
              "unlock.milestone.furnitureAssembler": 1,
              "unlock.upgrade.plasticizer": 1,
              "unlock.upgrade.furnitureassembler": 1,
              "unlock.upgrade.coalpower": 1,
              "unlock.upgrade.steelmill": 1,
              "unlock.c1.m10a1": 1,
              "unlock.c1.m10b1": 1,
              "unlock.milestone.oilrefinery": 1,
              "unlock.milestone.neonrefinery": 1,
              "unlock.milestone.goodsFactory": 1,
              "unlock.milestone.hardmold": 1,
              "unlock.milestone.electronicsFab": 1,
              "unlock.c1.m11": 1,
              "unlock.c1.m10bonus": 1,
              "unlock.milestone.incinerator": 1,
              "unlock.milestone.oreGassifier": 1,
              "unlock.c1.mspecial3": 1,
              "unlock.c1.m11bonus1": 1,
              "unlock.c1.m11bonus2": 1,
              "unlock.placeable.electronicsfab": 1,
              "unlock.upgrade.goodsfactory": 1,
              "unlock.c1.m12": 1,
              "unlock.milestone.tooldie": 1,
              "unlock.c1.m12bonus": 1,
              "unlock.c1.m13": 1,
              "unlock.upgrade.tooldie": 1,
              "unlock.upgrade.oilrefinery": 1,
              "unlock.c1.m10a2": 1,
              "unlock.milestone.Electric": 1,
              "unlock.c1.m13bonus1": 1,
              "unlock.c1.m13bonus2": 1,
              "unlock.c1.m14": 1,
              "unlock.milestone.hullyard": 1,
              "unlock.milestone.MaxHeight": 1,
              "unlock.HybridEngine": 1,
              "unlock.upgrade.neonrefinery": 1,
              "unlock.upgrade.electronicsfab": 1,
              "unlock.upgrade.hullyard": 1,
              "unlock.upgrade.incinerator": 1,
              "unlock.upgrade.oregassifier": 1,
              "unlock.upgrade.hardmold": 1,
              "unlock.milestone.luxuryAssembler": 1,
              "unlock.c1.mspecial4": 1,
              "unlock.c1.m14bonus": 1,
              "unlock.milestone.rescue": 1,
              "unlock.c1.m15": 1,
              "unlock.milestone.glassSmelter": 1,
              "unlock.milestone.signworks": 1,
              "unlock.c1.m16": 1,
              "unlock.c1.m15bonus1": 1,
              "unlock.c1.m15bonus2": 1,
              "unlock.milestone.chipfab": 1,
              "unlock.upgrade.glasssmelter": 1,
              "unlock.stationSpeed": 1,
              "unlock.c1.m17a": 1,
              "unlock.c1.m17b": 1,
              "unlock.c1.m16bonus": 1,
              "unlock.milestone.tubePlant": 1,
              "unlock.milestone.diesel": 1,
              "unlock.milestone.motor": 1,
              "unlock.upgrade.motorAssembly": 1,
              "unlock.milestone.heavyworks": 1,
              "unlock.milestone.framer": 1,
              "unlock.c1.m18a": 1,
              "unlock.c1.m18b": 1,
              "unlock.c1.m17bonus": 1,
              "placeholder": 1,
              "unlock.c1.m4bonus": 1,
              "unlock.c1.m5": 1,
              "unlock.c1.m4bonus2": 1,
              "unlock.DieselEngine": 1,
              "unlock.upgrade.signworks": 1,
              "unlock.upgrade.chipfab": 1,
              "unlock.RescueEngine": 1,
              "unlock.upgrade.tubeplant": 1,
              "unlock.upgrade.luxuryassembler": 1,
              "unlock.upgrade.framer": 1,
              "unlock.upgrade.heavyworks": 1,
              "unlock.milestone.armory": 1,
              "unlock.milestone.boosterplant": 1,
              "unlock.milestone.rocketfactory": 1,
              "unlock.milestone.Bullet": 1,
              "unlock.c1.m19a": 1,
              "unlock.c1.m19b": 1,
              "unlock.c1.m18bonus": 1,
              "unlock.BulletEngine": 1,
              "unlock.upgrade.rocketfactory": 1
            }
          }
        ],
        "historicCounts": [
          {
            "type": "MapProperty",
            "name_ints": {
              "unlock.c1.t2": 1,
              "unlock.c1.t3": 1,
              "unlock.c1.m1": 1,
              "unlock.c1.m2": 1,
              "unlock.milestone.oilwell": 1,
              "unlock.upgrade.oilpower": 1,
              "unlock.milestone.Workhorse": 1,
              "unlock.c1.m2bonus": 1,
              "unlock.c1.m3": 1,
              "unlock.CustomEngine": 1,
              "unlock.placeable.geothermal": 1,
              "unlock.upgrade.geothermal": 1,
              "unlock.milestone.oilPower": 1,
              "unlock.c1.m4": 1,
              "unlock.milestone.coppermine": 1,
              "unlock.milestone.coiler": 1,
              "unlock.milestone.limestonemine": 1,
              "unlock.milestone.concreteYard": 1,
              "unlock.upgrade.concreteyard": 1,
              "unlock.upgrade.coiler": 1,
              "unlock.c1.mspecial1": 1,
              "unlock.milestone.autoBranching": 1,
              "unlock.c1.m6": 1,
              "unlock.milestone.MediumHeight": 1,
              "unlock.c1.m7": 1,
              "unlock.milestone.saltsifter": 1,
              "unlock.milestone.coalMine": 1,
              "unlock.upgrade.saltsifter": 1,
              "unlock.c1.m8": 1,
              "unlock.milestone.coalpower": 1,
              "unlock.milestone.Industrial": 1,
              "unlock.milestone.ironmine": 1,
              "unlock.c1.m9": 1,
              "unlock.c1.m8bonus1": 1,
              "unlock.c1.m8bonus2": 1,
              "unlock.milestone.steelMill": 1,
              "unlock.milestone.plasticizer": 1,
              "unlock.c1.mspecial2": 1,
              "unlock.c1.m9bonus": 1,
              "unlock.milestone.furnitureAssembler": 1,
              "unlock.upgrade.plasticizer": 1,
              "unlock.upgrade.furnitureassembler": 1,
              "unlock.upgrade.coalpower": 1,
              "unlock.upgrade.steelmill": 1,
              "unlock.c1.m10a1": 1,
              "unlock.c1.m10b1": 1,
              "unlock.milestone.oilrefinery": 1,
              "unlock.milestone.neonrefinery": 1,
              "unlock.milestone.goodsFactory": 1,
              "unlock.milestone.hardmold": 1,
              "unlock.milestone.electronicsFab": 1,
              "unlock.c1.m11": 1,
              "unlock.c1.m10bonus": 1,
              "unlock.milestone.incinerator": 1,
              "unlock.milestone.oreGassifier": 1,
              "unlock.c1.mspecial3": 1,
              "unlock.c1.m11bonus1": 1,
              "unlock.c1.m11bonus2": 1,
              "unlock.placeable.electronicsfab": 1,
              "unlock.upgrade.goodsfactory": 1,
              "unlock.c1.m12": 1,
              "unlock.milestone.tooldie": 1,
              "unlock.c1.m12bonus": 1,
              "unlock.c1.m13": 1,
              "unlock.upgrade.tooldie": 1,
              "unlock.upgrade.oilrefinery": 1,
              "unlock.c1.m10a2": 1,
              "unlock.milestone.Electric": 1,
              "unlock.c1.m13bonus1": 1,
              "unlock.c1.m13bonus2": 1,
              "unlock.c1.m14": 1,
              "unlock.milestone.hullyard": 1,
              "unlock.milestone.MaxHeight": 1,
              "unlock.HybridEngine": 1,
              "unlock.upgrade.neonrefinery": 1,
              "unlock.upgrade.electronicsfab": 1,
              "unlock.upgrade.hullyard": 1,
              "unlock.upgrade.incinerator": 1,
              "unlock.upgrade.oregassifier": 1,
              "unlock.upgrade.hardmold": 1,
              "unlock.milestone.luxuryAssembler": 1,
              "unlock.c1.mspecial4": 1,
              "unlock.c1.m14bonus": 1,
              "unlock.milestone.rescue": 1,
              "unlock.c1.m15": 1,
              "unlock.milestone.glassSmelter": 1,
              "unlock.milestone.signworks": 1,
              "unlock.c1.m16": 1,
              "unlock.c1.m15bonus1": 1,
              "unlock.c1.m15bonus2": 1,
              "unlock.milestone.chipfab": 1,
              "unlock.upgrade.glasssmelter": 1,
              "unlock.stationSpeed": 1,
              "unlock.c1.m17a": 1,
              "unlock.c1.m17b": 1,
              "unlock.c1.m16bonus": 1,
              "unlock.milestone.tubePlant": 1,
              "unlock.milestone.diesel": 1,
              "unlock.milestone.motor": 1,
              "unlock.upgrade.motorAssembly": 1,
              "unlock.milestone.heavyworks": 1,
              "unlock.milestone.framer": 1,
              "unlock.c1.m18a": 1,
              "unlock.c1.m18b": 1,
              "unlock.c1.m17bonus": 1,
              "placeholder": 1,
              "unlock.c1.m4bonus": 1,
              "unlock.c1.m5": 1,
              "unlock.c1.m4bonus2": 1,
              "unlock.DieselEngine": 1,
              "unlock.upgrade.signworks": 1,
              "unlock.upgrade.chipfab": 1,
              "unlock.RescueEngine": 1,
              "unlock.upgrade.tubeplant": 1,
              "unlock.upgrade.luxuryassembler": 1,
              "unlock.upgrade.framer": 1,
              "unlock.upgrade.heavyworks": 1,
              "unlock.milestone.armory": 1,
              "unlock.milestone.boosterplant": 1,
              "unlock.milestone.rocketfactory": 1,
              "unlock.milestone.Bullet": 1,
              "unlock.c1.m19a": 1,
              "unlock.c1.m19b": 1,
              "unlock.c1.m18bonus": 1,
              "unlock.BulletEngine": 1,
              "unlock.upgrade.rocketfactory": 1
            }
          }
        ]
      }
    },
    "achievementHistoryScope": {
      "type": "StructProperty",
      "type_name": "MetaMetricStorageScope",
      "CustomStruct": {
        "metrics": [
          {
            "type": "MapProperty",
            "value_type": "StructProperty",
            "name_props": {
              "Profit": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 737
                      }
                    }
                  ]
                }
              },
              "export.rate": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "Energy": 0,
                        "Mainframes": 0
                      }
                    }
                  ]
                }
              },
              "produce": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "Energy": 1043,
                        "CrudeOil": 1047
                      }
                    }
                  ]
                }
              },
              "cycle": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "placeable.geothermal": 70
                      }
                    }
                  ]
                }
              },
              "build.track": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 8341
                      }
                    }
                  ]
                }
              },
              "build.branch": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 1848
                      }
                    }
                  ]
                }
              },
              "build.train": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 1120
                      }
                    }
                  ]
                }
              },
              "arrive.length": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 13
                      }
                    }
                  ]
                }
              },
              "arrive.freight": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "Water": 2067,
                        "Steel": 2059
                      }
                    }
                  ]
                }
              },
              "salvage.track": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 1128
                      }
                    }
                  ]
                }
              },
              "salvage.branch": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 59
                      }
                    }
                  ]
                }
              },
              "upgrade": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 268,
                        "placeable.waterpump": 32
                      }
                    }
                  ]
                }
              },
              "Path": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "None": 61
                      }
                    }
                  ]
                }
              },
              "mission": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "82A9B46F42BF6C5555D608A6619B35E6": 2
                      }
                    }
                  ]
                }
              },
              "maxupgrade": {
                "type": "StructPropertyValue",
                "CustomStruct": {
                  "valueByFilter": [
                    {
                      "type": "MapProperty",
                      "name_ints": {
                        "city": 2
                      }
                    }
                  ]
                }
              }
            }
          }
        ]
      }
    }
  }
}"#;
