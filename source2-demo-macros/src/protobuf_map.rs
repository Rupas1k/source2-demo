use quote::quote;

pub fn get_enum_from_struct(struct_name: &str) -> proc_macro2::TokenStream {
    match struct_name {
        // EDemoCommands
        "CDemoFileHeader" => quote! { EDemoCommands::DemFileHeader },
        // "CDemoError" => quote! { EDemoCommands::DemError },
        "CDemoStop" => quote! { EDemoCommands::DemStop },
        "CDemoFileInfo" => quote! { EDemoCommands::DemFileInfo },
        "CDemoSyncTick" => quote! { EDemoCommands::DemSyncTick },
        "CDemoSendTables" => quote! { EDemoCommands::DemSendTables },
        "CDemoClassInfo" => quote! { EDemoCommands::DemClassInfo },
        "CDemoStringTables" => quote! { EDemoCommands::DemStringTables },
        "CDemoPacket" => quote! { EDemoCommands::DemPacket },
        // "CDemoSignonPacket" => quote! { EDemoCommands::DemSignonPacket },
        "CDemoConsoleCmd" => quote! { EDemoCommands::DemConsoleCmd },
        "CDemoCustomData" => quote! { EDemoCommands::DemCustomData },
        "CDemoCustomDataCallbacks" => quote! { EDemoCommands::DemCustomDataCallbacks },
        "CDemoUserCmd" => quote! { EDemoCommands::DemUserCmd },
        "CDemoFullPacket" => quote! { EDemoCommands::DemFullPacket },
        "CDemoSaveGame" => quote! { EDemoCommands::DemSaveGame },
        "CDemoSpawnGroups" => quote! { EDemoCommands::DemSpawnGroups },
        "CDemoAnimationData" => quote! { EDemoCommands::DemAnimationData },
        "CDemoAnimationHeader" => quote! { EDemoCommands::DemAnimationHeader },
        // "CDemoMax" => quote! { EDemoCommands::DemMax },
        // "CDemoIsCompressed" => quote! { EDemoCommands::DemIsCompressed },

        // CitadelUserMessageIds
        "CCitadelUserMessageDamage" => quote! { CitadelUserMessageIds::KEUserMsgDamage },
        "CCitadelUserMsgMapPing" => quote! { CitadelUserMessageIds::KEUserMsgMapPing },
        "CCitadelUserMsgTeamRewards" => quote! { CitadelUserMessageIds::KEUserMsgTeamRewards },
        // "CCitadelUserMsgAbilityFailed" => quote! { CitadelUserMessageIds::KEUserMsgAbilityFailed
        // },
        "CCitadelUserMsgTriggerDamageFlash" => { quote! { CitadelUserMessageIds::KEUserMsgTriggerDamageFlash }
        }
        "CCitadelUserMsgAbilitiesChanged" => {
            quote! { CitadelUserMessageIds::KEUserMsgAbilitiesChanged }
        }
        "CCitadelUserMsgRecentDamageSummary" => {
            quote! { CitadelUserMessageIds::KEUserMsgRecentDamageSummary }
        }
        "CCitadelUserMsgSpectatorTeamChanged" => {
            quote! { CitadelUserMessageIds::KEUserMsgSpectatorTeamChanged }
        }
        "CCitadelUserMsgChatWheel" => quote! { CitadelUserMessageIds::KEUserMsgChatWheel },
        "CCitadelUserMsgGoldHistory" => quote! { CitadelUserMessageIds::KEUserMsgGoldHistory },
        "CCitadelUserMsgChatMsg" => quote! { CitadelUserMessageIds::KEUserMsgChatMsg },
        "CCitadelUserMsgQuickResponse" => quote! { CitadelUserMessageIds::KEUserMsgQuickResponse },
        "CCitadelUserMsgPostMatchDetails" => {
            quote! { CitadelUserMessageIds::KEUserMsgPostMatchDetails }
        }
        "CCitadelUserMsgChatEvent" => quote! { CitadelUserMessageIds::KEUserMsgChatEvent },
        "CCitadelUserMsgAbilityInterrupted" => {
            quote! { CitadelUserMessageIds::KEUserMsgAbilityInterrupted }
        }
        "CCitadelUserMsgHeroKilled" => quote! { CitadelUserMessageIds::KEUserMsgHeroKilled },
        "CCitadelUserMsgReturnIdol" => quote! { CitadelUserMessageIds::KEUserMsgReturnIdol },
        "CCitadelUserMsgSetClientCameraAngles" => {
            quote! { CitadelUserMessageIds::KEUserMsgSetClientCameraAngles }
        }
        "CCitadelUserMsgMapLine" => quote! { CitadelUserMessageIds::KEUserMsgMapLine },
        "CCitadelUserMessageBulletHit" => quote! { CitadelUserMessageIds::KEUserMsgBulletHit },
        "CCitadelUserMessageObjectiveMask" => {
            quote! { CitadelUserMessageIds::KEUserMsgObjectiveMask }
        }
        "CCitadelUserMessageModifierApplied" => {
            quote! { CitadelUserMessageIds::KEUserMsgModifierApplied }
        }
        "CCitadelUserMsgCameraController" => {
            quote! { CitadelUserMessageIds::KEUserMsgCameraController }
        }
        "CCitadelUserMessageAuraModifierApplied" => {
            quote! { CitadelUserMessageIds::KEUserMsgAuraModifierApplied }
        }
        "CCitadelUserMsgObstructedShotFired" => {
            quote! { CitadelUserMessageIds::KEUserMsgObstructedShotFired }
        }
        "CCitadelUserMsgAbilityLateFailure" => {
            quote! { CitadelUserMessageIds::KEUserMsgAbilityLateFailure }
        }
        "CCitadelUserMsgAbilityPing" => quote! { CitadelUserMessageIds::KEUserMsgAbilityPing },
        "CCitadelUserMsgPostProcessingAnim" => {
            quote! { CitadelUserMessageIds::KEUserMsgPostProcessingAnim }
        }
        "CCitadelUserMsgDeathReplayData" => {
            quote! { CitadelUserMessageIds::KEUserMsgDeathReplayData }
        }
        "CCitadelUserMsgPlayerLifetimeStatInfo" => {
            quote! { CitadelUserMessageIds::KEUserMsgPlayerLifetimeStatInfo }
        }
        "CCitadelUserMsgForceShopClosed" => {
            quote! { CitadelUserMessageIds::KEUserMsgForceShopClosed }
        }
        "CCitadelUserMsgStaminaDrained" => {
            quote! { CitadelUserMessageIds::KEUserMsgStaminaDrained }
        }
        "CCitadelUserMessageAbilityNotify" => {
            quote! { CitadelUserMessageIds::KEUserMsgAbilityNotify }
        }
        "CCitadelUserMsgGetDamageStatsResponse" => {
            quote! { CitadelUserMessageIds::KEUserMsgGetDamageStatsResponse }
        }
        "CCitadelUserMsgParticipantStartSoundEvent" => {
            quote! { CitadelUserMessageIds::KEUserMsgParticipantStartSoundEvent }
        }
        "CCitadelUserMsgParticipantStopSoundEvent" => {
            quote! { CitadelUserMessageIds::KEUserMsgParticipantStopSoundEvent }
        }
        "CCitadelUserMsgParticipantStopSoundEventHash" => {
            quote! { CitadelUserMessageIds::KEUserMsgParticipantStopSoundEventHash }
        }
        "CCitadelUserMsgParticipantSetSoundEventParams" => {
            quote! { CitadelUserMessageIds::KEUserMsgParticipantSetSoundEventParams }
        }
        "CCitadelUserMsgParticipantSetLibraryStackFields" => {
            quote! { CitadelUserMessageIds::KEUserMsgParticipantSetLibraryStackFields }
        }
        "CCitadelUserMessageCurrencyChanged" => {
            quote! { CitadelUserMessageIds::KEUserMsgCurrencyChanged }
        }
        "CCitadelUserMessageGameOver" => quote! { CitadelUserMessageIds::KEUserMsgGameOver },

        // ECitadelGameEvents
        "CMsgFireBullets" => quote! { ECitadelGameEvents::GeFireBullets },
        "CMsgPlayerAnimEvent" => quote! { ECitadelGameEvents::GePlayerAnimEvent },
        "CMsgParticleSystemManager" => quote! { ECitadelGameEvents::GeParticleSystemManager },
        "CMsgScreenTextPretty" => quote! { ECitadelGameEvents::GeScreenTextPretty },
        "CMsgServerRequestedTracer" => quote! { ECitadelGameEvents::GeServerRequestedTracer },
        "CMsgBulletImpact" => quote! { ECitadelGameEvents::GeBulletImpact },
        "CMsgEnableSatVolumesEvent" => quote! { ECitadelGameEvents::GeEnableSatVolumesEvent },
        "CMsgPlaceSatVolumeEvent" => quote! { ECitadelGameEvents::GePlaceSatVolumeEvent },
        "CMsgDisableSatVolumesEvent" => quote! { ECitadelGameEvents::GeDisableSatVolumesEvent },
        "CMsgRemoveSatVolumeEvent" => quote! { ECitadelGameEvents::GeRemoveSatVolumeEvent },

        // EDotaUserMessages

        // "CDotaUserMsgAddUnitToSelection" => quote! { EDotaUserMessages::DotaUmAddUnitToSelection
        // },
        "CDotaUserMsgAiDebugLine" => quote! { EDotaUserMessages::DotaUmAiDebugLine },
        "CDotaUserMsgChatEvent" => quote! { EDotaUserMessages::DotaUmChatEvent },
        "CDotaUserMsgCombatHeroPositions" => {
            quote! { EDotaUserMessages::DotaUmCombatHeroPositions }
        }
        // "CDotaUserMsgCombatLogData" => quote! { EDotaUserMessages::DotaUmCombatLogData },
        "CDotaUserMsgCombatLogBulkData" => quote! { EDotaUserMessages::DotaUmCombatLogBulkData },
        "CDotaUserMsgCreateLinearProjectile" => {
            quote! { EDotaUserMessages::DotaUmCreateLinearProjectile }
        }
        "CDotaUserMsgDestroyLinearProjectile" => {
            quote! { EDotaUserMessages::DotaUmDestroyLinearProjectile }
        }
        "CDotaUserMsgDodgeTrackingProjectiles" => {
            quote! { EDotaUserMessages::DotaUmDodgeTrackingProjectiles }
        }
        "CDotaUserMsgGlobalLightColor" => quote! { EDotaUserMessages::DotaUmGlobalLightColor },
        "CDotaUserMsgGlobalLightDirection" => {
            quote! { EDotaUserMessages::DotaUmGlobalLightDirection }
        }
        "CDotaUserMsgInvalidCommand" => quote! { EDotaUserMessages::DotaUmInvalidCommand },
        "CDotaUserMsgLocationPing" => quote! { EDotaUserMessages::DotaUmLocationPing },
        "CDotaUserMsgMapLine" => quote! { EDotaUserMessages::DotaUmMapLine },
        "CDotaUserMsgMiniKillCamInfo" => quote! { EDotaUserMessages::DotaUmMiniKillCamInfo },
        "CDotaUserMsgMinimapDebugPoint" => quote! { EDotaUserMessages::DotaUmMinimapDebugPoint },
        "CDotaUserMsgMinimapEvent" => quote! { EDotaUserMessages::DotaUmMinimapEvent },
        "CDotaUserMsgNevermoreRequiem" => quote! { EDotaUserMessages::DotaUmNevermoreRequiem },
        "CDotaUserMsgOverheadEvent" => quote! { EDotaUserMessages::DotaUmOverheadEvent },
        "CDotaUserMsgSetNextAutobuyItem" => quote! { EDotaUserMessages::DotaUmSetNextAutobuyItem },
        "CDotaUserMsgSharedCooldown" => quote! { EDotaUserMessages::DotaUmSharedCooldown },
        "CDotaUserMsgSpectatorPlayerClick" => {
            quote! { EDotaUserMessages::DotaUmSpectatorPlayerClick }
        }
        "CDotaUserMsgTutorialTipInfo" => quote! { EDotaUserMessages::DotaUmTutorialTipInfo },
        "CDotaUserMsgUnitEvent" => quote! { EDotaUserMessages::DotaUmUnitEvent },
        // "CDotaUserMsgParticleManager" => quote! { EDotaUserMessages::DotaUmParticleManager },
        "CDotaUserMsgBotChat" => quote! { EDotaUserMessages::DotaUmBotChat },
        "CDotaUserMsgHudError" => quote! { EDotaUserMessages::DotaUmHudError },
        "CDotaUserMsgItemPurchased" => quote! { EDotaUserMessages::DotaUmItemPurchased },
        "CDotaUserMsgPing" => quote! { EDotaUserMessages::DotaUmPing },
        "CDotaUserMsgItemFound" => quote! { EDotaUserMessages::DotaUmItemFound },
        // "CDotaUserMsgCharacterSpeakConcept" => quote! {
        // EDotaUserMessages::DotaUmCharacterSpeakConcept },
        "CDotaUserMsgSwapVerify" => quote! { EDotaUserMessages::DotaUmSwapVerify },
        "CDotaUserMsgWorldLine" => quote! { EDotaUserMessages::DotaUmWorldLine },
        // "CDotaUserMsgTournamentDrop" => quote! { EDotaUserMessages::DotaUmTournamentDrop },
        "CDotaUserMsgItemAlert" => quote! { EDotaUserMessages::DotaUmItemAlert },
        "CDotaUserMsgHalloweenDrops" => quote! { EDotaUserMessages::DotaUmHalloweenDrops },
        "CDotaUserMsgChatWheel" => quote! { EDotaUserMessages::DotaUmChatWheel },
        "CDotaUserMsgReceivedXmasGift" => quote! { EDotaUserMessages::DotaUmReceivedXmasGift },
        "CDotaUserMsgUpdateSharedContent" => {
            quote! { EDotaUserMessages::DotaUmUpdateSharedContent }
        }
        "CDotaUserMsgTutorialRequestExp" => quote! { EDotaUserMessages::DotaUmTutorialRequestExp },
        "CDotaUserMsgTutorialPingMinimap" => {
            quote! { EDotaUserMessages::DotaUmTutorialPingMinimap }
        }
        "CDotaUserMsgGamerulesStateChanged" => {
            quote! { EDotaUserMessages::DotaUmGamerulesStateChanged }
        }
        "CDotaUserMsgShowSurvey" => quote! { EDotaUserMessages::DotaUmShowSurvey },
        "CDotaUserMsgTutorialFade" => quote! { EDotaUserMessages::DotaUmTutorialFade },
        "CDotaUserMsgAddQuestLogEntry" => quote! { EDotaUserMessages::DotaUmAddQuestLogEntry },
        "CDotaUserMsgSendStatPopup" => quote! { EDotaUserMessages::DotaUmSendStatPopup },
        "CDotaUserMsgTutorialFinish" => quote! { EDotaUserMessages::DotaUmTutorialFinish },
        "CDotaUserMsgSendRoshanPopup" => quote! { EDotaUserMessages::DotaUmSendRoshanPopup },
        "CDotaUserMsgSendGenericToolTip" => quote! { EDotaUserMessages::DotaUmSendGenericToolTip },
        "CDotaUserMsgSendFinalGold" => quote! { EDotaUserMessages::DotaUmSendFinalGold },
        "CDotaUserMsgCustomMsg" => quote! { EDotaUserMessages::DotaUmCustomMsg },
        "CDotaUserMsgCoachHudPing" => quote! { EDotaUserMessages::DotaUmCoachHudPing },
        "CDotaUserMsgClientLoadGridNav" => quote! { EDotaUserMessages::DotaUmClientLoadGridNav },
        "CDotaUserMsgTeProjectile" => quote! { EDotaUserMessages::DotaUmTeProjectile },
        "CDotaUserMsgTeProjectileLoc" => quote! { EDotaUserMessages::DotaUmTeProjectileLoc },
        "CDotaUserMsgTeDotaBloodImpact" => quote! { EDotaUserMessages::DotaUmTeDotaBloodImpact },
        "CDotaUserMsgTeUnitAnimation" => quote! { EDotaUserMessages::DotaUmTeUnitAnimation },
        "CDotaUserMsgTeUnitAnimationEnd" => quote! { EDotaUserMessages::DotaUmTeUnitAnimationEnd },
        "CDotaUserMsgAbilityPing" => quote! { EDotaUserMessages::DotaUmAbilityPing },
        "CDotaUserMsgShowGenericPopup" => quote! { EDotaUserMessages::DotaUmShowGenericPopup },
        "CDotaUserMsgVoteStart" => quote! { EDotaUserMessages::DotaUmVoteStart },
        "CDotaUserMsgVoteUpdate" => quote! { EDotaUserMessages::DotaUmVoteUpdate },
        "CDotaUserMsgVoteEnd" => quote! { EDotaUserMessages::DotaUmVoteEnd },
        "CDotaUserMsgBoosterState" => quote! { EDotaUserMessages::DotaUmBoosterState },
        "CDotaUserMsgWillPurchaseAlert" => quote! { EDotaUserMessages::DotaUmWillPurchaseAlert },
        "CDotaUserMsgTutorialMinimapPosition" => {
            quote! { EDotaUserMessages::DotaUmTutorialMinimapPosition }
        }
        "CDotaUserMsgPlayerMmr" => quote! { EDotaUserMessages::DotaUmPlayerMmr },
        "CDotaUserMsgAbilitySteal" => quote! { EDotaUserMessages::DotaUmAbilitySteal },
        "CDotaUserMsgCourierKilledAlert" => quote! { EDotaUserMessages::DotaUmCourierKilledAlert },
        "CDotaUserMsgEnemyItemAlert" => quote! { EDotaUserMessages::DotaUmEnemyItemAlert },
        "CDotaUserMsgStatsMatchDetails" => quote! { EDotaUserMessages::DotaUmStatsMatchDetails },
        "CDotaUserMsgMiniTaunt" => quote! { EDotaUserMessages::DotaUmMiniTaunt },
        "CDotaUserMsgBuyBackStateAlert" => quote! { EDotaUserMessages::DotaUmBuyBackStateAlert },
        "CDotaUserMsgSpeechBubble" => quote! { EDotaUserMessages::DotaUmSpeechBubble },
        "CDotaUserMsgCustomHeaderMessage" => {
            quote! { EDotaUserMessages::DotaUmCustomHeaderMessage }
        }
        "CDotaUserMsgQuickBuyAlert" => quote! { EDotaUserMessages::DotaUmQuickBuyAlert },
        "CDotaUserMsgStatsHeroDetails" => quote! { EDotaUserMessages::DotaUmStatsHeroDetails },
        "CDotaUserMsgPredictionResult" => quote! { EDotaUserMessages::DotaUmPredictionResult },
        "CDotaUserMsgModifierAlert" => quote! { EDotaUserMessages::DotaUmModifierAlert },
        "CDotaUserMsgHpManaAlert" => quote! { EDotaUserMessages::DotaUmHpManaAlert },
        "CDotaUserMsgGlyphAlert" => quote! { EDotaUserMessages::DotaUmGlyphAlert },
        "CDotaUserMsgBeastChat" => quote! { EDotaUserMessages::DotaUmBeastChat },
        "CDotaUserMsgSpectatorPlayerUnitOrders" => {
            quote! { EDotaUserMessages::DotaUmSpectatorPlayerUnitOrders }
        }
        "CDotaUserMsgCustomHudElementCreate" => {
            quote! { EDotaUserMessages::DotaUmCustomHudElementCreate }
        }
        "CDotaUserMsgCustomHudElementModify" => {
            quote! { EDotaUserMessages::DotaUmCustomHudElementModify }
        }
        "CDotaUserMsgCustomHudElementDestroy" => {
            quote! { EDotaUserMessages::DotaUmCustomHudElementDestroy }
        }
        "CDotaUserMsgCompendiumState" => quote! { EDotaUserMessages::DotaUmCompendiumState },
        "CDotaUserMsgProjectionAbility" => quote! { EDotaUserMessages::DotaUmProjectionAbility },
        "CDotaUserMsgProjectionEvent" => quote! { EDotaUserMessages::DotaUmProjectionEvent },
        "CMsgDotaCombatLogEntry" => quote! { EDotaUserMessages::DotaUmCombatLogDataHltv },
        "CDotaUserMsgXpAlert" => quote! { EDotaUserMessages::DotaUmXpAlert },
        "CDotaUserMsgUpdateQuestProgress" => {
            quote! { EDotaUserMessages::DotaUmUpdateQuestProgress }
        }
        // "CDotaUserMsgMatchMetadata" => quote! { EDotaUserMessages::DotaUmMatchMetadata },
        // "CDotaUserMsgMatchDetails" => quote! { EDotaUserMessages::DotaUmMatchDetails },
        "CDotaUserMsgQuestStatus" => quote! { EDotaUserMessages::DotaUmQuestStatus },
        "CDotaUserMsgSuggestHeroPick" => quote! { EDotaUserMessages::DotaUmSuggestHeroPick },
        "CDotaUserMsgSuggestHeroRole" => quote! { EDotaUserMessages::DotaUmSuggestHeroRole },
        "CDotaUserMsgKillcamDamageTaken" => quote! { EDotaUserMessages::DotaUmKillcamDamageTaken },
        "CDotaUserMsgSelectPenaltyGold" => quote! { EDotaUserMessages::DotaUmSelectPenaltyGold },
        "CDotaUserMsgRollDiceResult" => quote! { EDotaUserMessages::DotaUmRollDiceResult },
        "CDotaUserMsgFlipCoinResult" => quote! { EDotaUserMessages::DotaUmFlipCoinResult },
        "CDotaUserMsgRequestItemSuggestions" => {
            quote! { EDotaUserMessages::DotaUmRequestItemSuggestions }
        }
        // "CDotaUserMsgTeamCaptainChanged" => quote! { EDotaUserMessages::DotaUmTeamCaptainChanged
        // },
        "CDotaUserMsgSendRoshanSpectatorPhase" => {
            quote! { EDotaUserMessages::DotaUmSendRoshanSpectatorPhase }
        }
        "CDotaUserMsgChatWheelCooldown" => quote! { EDotaUserMessages::DotaUmChatWheelCooldown },
        "CDotaUserMsgDismissAllStatPopups" => {
            quote! { EDotaUserMessages::DotaUmDismissAllStatPopups }
        }
        "CDotaUserMsgTeDestroyProjectile" => {
            quote! { EDotaUserMessages::DotaUmTeDestroyProjectile }
        }
        "CDotaUserMsgHeroRelicProgress" => quote! { EDotaUserMessages::DotaUmHeroRelicProgress },
        "CDotaUserMsgAbilityDraftRequestAbility" => {
            quote! { EDotaUserMessages::DotaUmAbilityDraftRequestAbility }
        }
        "CDotaUserMsgItemSold" => quote! { EDotaUserMessages::DotaUmItemSold },
        "CDotaUserMsgDamageReport" => quote! { EDotaUserMessages::DotaUmDamageReport },
        "CDotaUserMsgSalutePlayer" => quote! { EDotaUserMessages::DotaUmSalutePlayer },
        "CDotaUserMsgTipAlert" => quote! { EDotaUserMessages::DotaUmTipAlert },
        "CDotaUserMsgReplaceQueryUnit" => quote! { EDotaUserMessages::DotaUmReplaceQueryUnit },
        "CDotaUserMsgEmptyTeleportAlert" => quote! { EDotaUserMessages::DotaUmEmptyTeleportAlert },
        "CDotaUserMsgMarsArenaOfBloodAttack" => {
            quote! { EDotaUserMessages::DotaUmMarsArenaOfBloodAttack }
        }
        "CDotaUserMsgEsArcanaCombo" => quote! { EDotaUserMessages::DotaUmEsArcanaCombo },
        "CDotaUserMsgEsArcanaComboSummary" => {
            quote! { EDotaUserMessages::DotaUmEsArcanaComboSummary }
        }
        "CDotaUserMsgHighFiveLeftHanging" => {
            quote! { EDotaUserMessages::DotaUmHighFiveLeftHanging }
        }
        "CDotaUserMsgHighFiveCompleted" => quote! { EDotaUserMessages::DotaUmHighFiveCompleted },
        "CDotaUserMsgShovelUnearth" => quote! { EDotaUserMessages::DotaUmShovelUnearth },
        "CDotaUserMsgInvokerSpellCast" => quote! { EDotaUserMessages::DotaEmInvokerSpellCast },
        "CDotaUserMsgRadarAlert" => quote! { EDotaUserMessages::DotaUmRadarAlert },
        "CDotaUserMsgAllStarEvent" => quote! { EDotaUserMessages::DotaUmAllStarEvent },
        "CDotaUserMsgTalentTreeAlert" => quote! { EDotaUserMessages::DotaUmTalentTreeAlert },
        "CDotaUserMsgQueuedOrderRemoved" => quote! { EDotaUserMessages::DotaUmQueuedOrderRemoved },
        "CDotaUserMsgDebugChallenge" => quote! { EDotaUserMessages::DotaUmDebugChallenge },
        "CDotaUserMsgOmArcanaCombo" => quote! { EDotaUserMessages::DotaUmOmArcanaCombo },
        "CDotaUserMsgFoundNeutralItem" => quote! { EDotaUserMessages::DotaUmFoundNeutralItem },
        "CDotaUserMsgOutpostCaptured" => quote! { EDotaUserMessages::DotaUmOutpostCaptured },
        "CDotaUserMsgOutpostGrantedXp" => quote! { EDotaUserMessages::DotaUmOutpostGrantedXp },
        "CDotaUserMsgMoveCameraToUnit" => quote! { EDotaUserMessages::DotaUmMoveCameraToUnit },
        "CDotaUserMsgPauseMinigameData" => quote! { EDotaUserMessages::DotaUmPauseMinigameData },
        "CDotaUserMsgVersusScenePlayerBehavior" => {
            quote! { EDotaUserMessages::DotaUmVersusScenePlayerBehavior }
        }
        "CDotaUserMsgQoPArcanaSummary" => quote! { EDotaUserMessages::DotaUmQoPArcanaSummary },
        "CDotaUserMsgHotPotatoCreated" => quote! { EDotaUserMessages::DotaUmHotPotatoCreated },
        "CDotaUserMsgHotPotatoExploded" => quote! { EDotaUserMessages::DotaUmHotPotatoExploded },
        "CDotaUserMsgWkArcanaProgress" => quote! { EDotaUserMessages::DotaUmWkArcanaProgress },
        "CDotaUserMsgGuildChallengeProgress" => {
            quote! { EDotaUserMessages::DotaUmGuildChallengeProgress }
        }
        "CDotaUserMsgWrArcanaProgress" => quote! { EDotaUserMessages::DotaUmWrArcanaProgress },
        "CDotaUserMsgWrArcanaSummary" => quote! { EDotaUserMessages::DotaUmWrArcanaSummary },
        "CDotaUserMsgEmptyItemSlotAlert" => quote! { EDotaUserMessages::DotaUmEmptyItemSlotAlert },
        "CDotaUserMsgAghsStatusAlert" => quote! { EDotaUserMessages::DotaUmAghsStatusAlert },
        "CDotaUserMsgPingConfirmation" => quote! { EDotaUserMessages::DotaUmPingConfirmation },
        "CDotaUserMsgMutedPlayers" => quote! { EDotaUserMessages::DotaUmMutedPlayers },
        "CDotaUserMsgContextualTip" => quote! { EDotaUserMessages::DotaUmContextualTip },
        "CDotaUserMsgChatMessage" => quote! { EDotaUserMessages::DotaUmChatMessage },
        "CDotaUserMsgNeutralCampAlert" => quote! { EDotaUserMessages::DotaUmNeutralCampAlert },
        "CDotaUserMsgRockPaperScissorsStarted" => {
            quote! { EDotaUserMessages::DotaUmRockPaperScissorsStarted }
        }
        "CDotaUserMsgRockPaperScissorsFinished" => {
            quote! { EDotaUserMessages::DotaUmRockPaperScissorsFinished }
        }
        "CDotaUserMsgDuelOpponentKilled" => quote! { EDotaUserMessages::DotaUmDuelOpponentKilled },
        "CDotaUserMsgDuelAccepted" => quote! { EDotaUserMessages::DotaUmDuelAccepted },
        "CDotaUserMsgDuelRequested" => quote! { EDotaUserMessages::DotaUmDuelRequested },
        "CDotaUserMsgMuertaReleaseEventAssignedTargetKilled" => {
            quote! { EDotaUserMessages::DotaUmMuertaReleaseEventAssignedTargetKilled }
        }
        "CDotaUserMsgPlayerDraftSuggestPick" => {
            quote! { EDotaUserMessages::DotaUmPlayerDraftSuggestPick }
        }
        "CDotaUserMsgPlayerDraftPick" => quote! { EDotaUserMessages::DotaUmPlayerDraftPick },
        "CDotaUserMsgUpdateLinearProjectileCpData" => {
            quote! { EDotaUserMessages::DotaUmUpdateLinearProjectileCpData }
        }
        "CDotaUserMsgGiftPlayer" => quote! { EDotaUserMessages::DotaUmGiftPlayer },
        "CDotaUserMsgFacetPing" => quote! { EDotaUserMessages::DotaUmFacetPing },
        "CDotaUserMsgInnatePing" => quote! { EDotaUserMessages::DotaUmInnatePing },

        // SvcMessages
        "CSvcMsgServerInfo" => quote! { SvcMessages::SvcServerInfo },
        "CSvcMsgFlattenedSerializer" => quote! { SvcMessages::SvcFlattenedSerializer },
        "CSvcMsgClassInfo" => quote! { SvcMessages::SvcClassInfo },
        "CSvcMsgSetPause" => quote! { SvcMessages::SvcSetPause },
        "CSvcMsgCreateStringTable" => quote! { SvcMessages::SvcCreateStringTable },
        "CSvcMsgUpdateStringTable" => quote! { SvcMessages::SvcUpdateStringTable },
        "CSvcMsgVoiceInit" => quote! { SvcMessages::SvcVoiceInit },
        "CSvcMsgVoiceData" => quote! { SvcMessages::SvcVoiceData },
        "CSvcMsgPrint" => quote! { SvcMessages::SvcPrint },
        "CSvcMsgSounds" => quote! { SvcMessages::SvcSounds },
        "CSvcMsgSetView" => quote! { SvcMessages::SvcSetView },
        "CSvcMsgClearAllStringTables" => quote! { SvcMessages::SvcClearAllStringTables },
        "CSvcMsgCmdKeyValues" => quote! { SvcMessages::SvcCmdKeyValues },
        "CSvcMsgBspDecal" => quote! { SvcMessages::SvcBspDecal },
        "CSvcMsgSplitScreen" => quote! { SvcMessages::SvcSplitScreen },
        "CSvcMsgPacketEntities" => quote! { SvcMessages::SvcPacketEntities },
        "CSvcMsgPrefetch" => quote! { SvcMessages::SvcPrefetch },
        "CSvcMsgMenu" => quote! { SvcMessages::SvcMenu },
        "CSvcMsgGetCvarValue" => quote! { SvcMessages::SvcGetCvarValue },
        "CSvcMsgStopSound" => quote! { SvcMessages::SvcStopSound },
        "CSvcMsgPeerList" => quote! { SvcMessages::SvcPeerList },
        "CSvcMsgPacketReliable" => quote! { SvcMessages::SvcPacketReliable },
        "CSvcMsgHltvStatus" => quote! { SvcMessages::SvcHltvStatus },
        "CSvcMsgServerSteamId" => quote! { SvcMessages::SvcServerSteamId },
        "CSvcMsgFullFrameSplit" => quote! { SvcMessages::SvcFullFrameSplit },
        "CSvcMsgRconServerDetails" => quote! { SvcMessages::SvcRconServerDetails },
        "CSvcMsgUserMessage" => quote! { SvcMessages::SvcUserMessage },
        "CSvcMsgHltvReplay" => quote! { SvcMessages::SvcHltvReplay },
        "CSvcMsgBroadcastCommand" => quote! { SvcMessages::SvcBroadcastCommand },
        "CSvcMsgHltvFixupOperatorStatus" => quote! { SvcMessages::SvcHltvFixupOperatorStatus },

        // EBaseUserMessages
        "CUserMessageAchievementEvent" => quote! { EBaseUserMessages::UmAchievementEvent },
        "CUserMessageCloseCaption" => quote! { EBaseUserMessages::UmCloseCaption },
        "CUserMessageCloseCaptionDirect" => quote! { EBaseUserMessages::UmCloseCaptionDirect },
        "CUserMessageCurrentTimescale" => quote! { EBaseUserMessages::UmCurrentTimescale },
        "CUserMessageDesiredTimescale" => quote! { EBaseUserMessages::UmDesiredTimescale },
        "CUserMessageFade" => quote! { EBaseUserMessages::UmFade },
        "CUserMessageGameTitle" => quote! { EBaseUserMessages::UmGameTitle },
        "CUserMessageHudMsg" => quote! { EBaseUserMessages::UmHudMsg },
        "CUserMessageHudText" => quote! { EBaseUserMessages::UmHudText },
        "CUserMessageColoredText" => quote! { EBaseUserMessages::UmColoredText },
        "CUserMessageRequestState" => quote! { EBaseUserMessages::UmRequestState },
        "CUserMessageResetHud" => quote! { EBaseUserMessages::UmResetHud },
        "CUserMessageRumble" => quote! { EBaseUserMessages::UmRumble },
        "CUserMessageSayText" => quote! { EBaseUserMessages::UmSayText },
        "CUserMessageSayText2" => quote! { EBaseUserMessages::UmSayText2 },
        "CUserMessageSayTextChannel" => quote! { EBaseUserMessages::UmSayTextChannel },
        "CUserMessageShake" => quote! { EBaseUserMessages::UmShake },
        "CUserMessageShakeDir" => quote! { EBaseUserMessages::UmShakeDir },
        "CUserMessageWaterShake" => quote! { EBaseUserMessages::UmWaterShake },
        "CUserMessageTextMsg" => quote! { EBaseUserMessages::UmTextMsg },
        "CUserMessageScreenTilt" => quote! { EBaseUserMessages::UmScreenTilt },
        "CUserMessageVoiceMask" => quote! { EBaseUserMessages::UmVoiceMask },
        "CUserMessageSendAudio" => quote! { EBaseUserMessages::UmSendAudio },
        "CUserMessageItemPickup" => quote! { EBaseUserMessages::UmItemPickup },
        "CUserMessageAmmoDenied" => quote! { EBaseUserMessages::UmAmmoDenied },
        "CUserMessageShowMenu" => quote! { EBaseUserMessages::UmShowMenu },
        "CUserMessageCreditsMsg" => quote! { EBaseUserMessages::UmCreditsMsg },
        "CUserMessageCloseCaptionPlaceholder" => {
            quote! { EBaseUserMessages::UmCloseCaptionPlaceholder }
        }
        "CUserMessageCameraTransition" => quote! { EBaseUserMessages::UmCameraTransition },
        "CUserMessageAudioParameter" => quote! { EBaseUserMessages::UmAudioParameter },
        "CUserMsgParticleManager" => quote! { EBaseUserMessages::UmParticleManager },
        "CUserMessageHudError" => quote! { EBaseUserMessages::UmHudError },
        "CUserMsgHudError" => quote! { EBaseUserMessages::UmCustomGameEvent },
        // "CUserMessageAnimGraphUpdate" => quote! { EBaseUserMessages::UmAnimGraphUpdate },
        "CUserMessageHapticsManagerPulse" => quote! { EBaseUserMessages::UmHapticsManagerPulse },
        "CUserMessageHapticsManagerEffect" => quote! { EBaseUserMessages::UmHapticsManagerEffect },
        // "CUserMessageCommandQueueState" => quote! { EBaseUserMessages::UmCommandQueueState },
        "CUserMessageUpdateCssClasses" => quote! { EBaseUserMessages::UmUpdateCssClasses },
        "CUserMessageServerFrameTime" => quote! { EBaseUserMessages::UmServerFrameTime },
        "CUserMessageLagCompensationError" => quote! { EBaseUserMessages::UmLagCompensationError },
        "CUserMessageRequestDllStatus" => quote! { EBaseUserMessages::UmRequestDllStatus },
        "CUserMessageRequestUtilAction" => quote! { EBaseUserMessages::UmRequestUtilAction },
        // "CUserMessageUtilActionResponse" => quote! { EBaseUserMessages::UmUtilActionResponse },
        // "CUserMessageDllStatusResponse" => quote! { EBaseUserMessages::UmDllStatusResponse },
        "CUserMessageRequestInventory" => quote! { EBaseUserMessages::UmRequestInventory },
        "CUserMessageInventoryResponse" => quote! { EBaseUserMessages::UmInventoryResponse },
        "CUserMessageRequestDiagnostic" => quote! { EBaseUserMessages::UmRequestDiagnostic },
        "CUserMessageDiagnosticResponse" => quote! { EBaseUserMessages::UmDiagnosticResponse },
        "CUserMessageExtraUserData" => quote! { EBaseUserMessages::UmExtraUserData },
        "CUserMessageNotifyResponseFound" => quote! { EBaseUserMessages::UmNotifyResponseFound },
        "CUserMessagePlayResponseConditional" => {
            quote! { EBaseUserMessages::UmPlayResponseConditional }
        }

        // EBaseGameEvents
        "CMsgSource1LegacyGameEventList" => {
            quote! { EBaseGameEvents::GeSource1LegacyGameEventList }
        }
        "CMsgSource1LegacyListenEvents" => quote! { EBaseGameEvents::GeSource1LegacyListenEvents },
        "CMsgSource1LegacyGameEvent" => quote! { EBaseGameEvents::GeSource1LegacyGameEvent },
        "CMsgSosStartSoundEvent" => quote! { EBaseGameEvents::GeSosStartSoundEvent },
        "CMsgSosStopSoundEvent" => quote! { EBaseGameEvents::GeSosStopSoundEvent },
        "CMsgSosSetSoundEventParams" => quote! { EBaseGameEvents::GeSosSetSoundEventParams },
        "CMsgSosSetLibraryStackFields" => quote! { EBaseGameEvents::GeSosSetLibraryStackFields },
        "CMsgSosStopSoundEventHash" => quote! { EBaseGameEvents::GeSosStopSoundEventHash },

        // NetMessages
        "CNetMsgNop" => quote! { NetMessages::NetNop },
        "CNetMsgDisconnectLegacy" => quote! { NetMessages::NetDisconnectLegacy },
        "CNetMsgSplitScreenUser" => quote! { NetMessages::NetSplitScreenUser },
        "CNetMsgTick" => quote! { NetMessages::NetTick },
        "CNetMsgStringCmd" => quote! { NetMessages::NetStringCmd },
        "CNetMsgSetConVar" => quote! { NetMessages::NetSetConVar },
        "CNetMsgSignonState" => quote! { NetMessages::NetSignonState },
        "CNetMsgSpawnGroupLoad" => quote! { NetMessages::NetSpawnGroupLoad },
        "CNetMsgSpawnGroupManifestUpdate" => quote! { NetMessages::NetSpawnGroupManifestUpdate },
        "CNetMsgSpawnGroupSetCreationTick" => quote! { NetMessages::NetSpawnGroupSetCreationTick },
        "CNetMsgSpawnGroupUnload" => quote! { NetMessages::NetSpawnGroupUnload },
        "CNetMsgSpawnGroupLoadCompleted" => quote! { NetMessages::NetSpawnGroupLoadCompleted },
        "CNetMsgDebugOverlay" => quote! { NetMessages::NetDebugOverlay },

        _ => panic!("Unknown message type: {}", struct_name),
    }
}
