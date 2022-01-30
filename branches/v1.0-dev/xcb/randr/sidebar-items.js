initSidebarItems({"constant":[["MAJOR_VERSION","The major version of the `RandR` extension."],["MINOR_VERSION","The minor version of the `RandR` extension."],["VERSION_STRING","The version string of the `RandR` extension."],["XNAME","The official identifier for the `RandR` extension."]],"enum":[["Connection",""],["Error","Unified error type for the RandR extension"],["Event","Unified event type for the RandR extension"],["Notify",""],["NotifyData",""],["SetConfig",""]],"fn":[["get_extension_data","Fetch server runtime info data of the `RandR` extension."],["prefetch_extension_data","Prefetch server runtime info data of the `RandR` extension."]],"struct":[["AddOutputMode","The `AddOutputMode` request."],["BadCrtcError","The `BadCrtcError` error."],["BadModeError","The `BadModeError` error."],["BadOutputError","The `BadOutputError` error."],["BadProviderError","The `BadProviderError` error."],["ChangeOutputProperty","The `ChangeOutputProperty` request."],["ChangeProviderProperty","The `ChangeProviderProperty` request."],["ConfigureOutputProperty","The `ConfigureOutputProperty` request."],["ConfigureProviderProperty","The `ConfigureProviderProperty` request."],["CreateLease","The `CreateLease` request."],["CreateLeaseCookie","Cookie type for [CreateLease]."],["CreateLeaseCookieUnchecked","Unchecked cookie type for [CreateLease]."],["CreateLeaseReply","Reply type for [CreateLease]."],["CreateMode","The `CreateMode` request."],["CreateModeCookie","Cookie type for [CreateMode]."],["CreateModeCookieUnchecked","Unchecked cookie type for [CreateMode]."],["CreateModeReply","Reply type for [CreateMode]."],["Crtc",""],["CrtcChange",""],["DeleteMonitor","The `DeleteMonitor` request."],["DeleteOutputMode","The `DeleteOutputMode` request."],["DeleteOutputProperty","The `DeleteOutputProperty` request."],["DeleteProviderProperty","The `DeleteProviderProperty` request."],["DestroyMode","The `DestroyMode` request."],["FreeLease","The `FreeLease` request."],["GetCrtcGamma","The `GetCrtcGamma` request."],["GetCrtcGammaCookie","Cookie type for [GetCrtcGamma]."],["GetCrtcGammaCookieUnchecked","Unchecked cookie type for [GetCrtcGamma]."],["GetCrtcGammaReply","Reply type for [GetCrtcGamma]."],["GetCrtcGammaSize","The `GetCrtcGammaSize` request."],["GetCrtcGammaSizeCookie","Cookie type for [GetCrtcGammaSize]."],["GetCrtcGammaSizeCookieUnchecked","Unchecked cookie type for [GetCrtcGammaSize]."],["GetCrtcGammaSizeReply","Reply type for [GetCrtcGammaSize]."],["GetCrtcInfo","The `GetCrtcInfo` request."],["GetCrtcInfoCookie","Cookie type for [GetCrtcInfo]."],["GetCrtcInfoCookieUnchecked","Unchecked cookie type for [GetCrtcInfo]."],["GetCrtcInfoReply","Reply type for [GetCrtcInfo]."],["GetCrtcTransform","The `GetCrtcTransform` request."],["GetCrtcTransformCookie","Cookie type for [GetCrtcTransform]."],["GetCrtcTransformCookieUnchecked","Unchecked cookie type for [GetCrtcTransform]."],["GetCrtcTransformReply","Reply type for [GetCrtcTransform]."],["GetMonitors","The `GetMonitors` request."],["GetMonitorsCookie","Cookie type for [GetMonitors]."],["GetMonitorsCookieUnchecked","Unchecked cookie type for [GetMonitors]."],["GetMonitorsReply","Reply type for [GetMonitors]."],["GetOutputInfo","The `GetOutputInfo` request."],["GetOutputInfoCookie","Cookie type for [GetOutputInfo]."],["GetOutputInfoCookieUnchecked","Unchecked cookie type for [GetOutputInfo]."],["GetOutputInfoReply","Reply type for [GetOutputInfo]."],["GetOutputPrimary","The `GetOutputPrimary` request."],["GetOutputPrimaryCookie","Cookie type for [GetOutputPrimary]."],["GetOutputPrimaryCookieUnchecked","Unchecked cookie type for [GetOutputPrimary]."],["GetOutputPrimaryReply","Reply type for [GetOutputPrimary]."],["GetOutputProperty","The `GetOutputProperty` request."],["GetOutputPropertyCookie","Cookie type for [GetOutputProperty]."],["GetOutputPropertyCookieUnchecked","Unchecked cookie type for [GetOutputProperty]."],["GetOutputPropertyReply","Reply type for [GetOutputProperty]."],["GetPanning","The `GetPanning` request."],["GetPanningCookie","Cookie type for [GetPanning]."],["GetPanningCookieUnchecked","Unchecked cookie type for [GetPanning]."],["GetPanningReply","Reply type for [GetPanning]."],["GetProviderInfo","The `GetProviderInfo` request."],["GetProviderInfoCookie","Cookie type for [GetProviderInfo]."],["GetProviderInfoCookieUnchecked","Unchecked cookie type for [GetProviderInfo]."],["GetProviderInfoReply","Reply type for [GetProviderInfo]."],["GetProviderProperty","The `GetProviderProperty` request."],["GetProviderPropertyCookie","Cookie type for [GetProviderProperty]."],["GetProviderPropertyCookieUnchecked","Unchecked cookie type for [GetProviderProperty]."],["GetProviderPropertyReply","Reply type for [GetProviderProperty]."],["GetProviders","The `GetProviders` request."],["GetProvidersCookie","Cookie type for [GetProviders]."],["GetProvidersCookieUnchecked","Unchecked cookie type for [GetProviders]."],["GetProvidersReply","Reply type for [GetProviders]."],["GetScreenInfo","The `GetScreenInfo` request."],["GetScreenInfoCookie","Cookie type for [GetScreenInfo]."],["GetScreenInfoCookieUnchecked","Unchecked cookie type for [GetScreenInfo]."],["GetScreenInfoReply","Reply type for [GetScreenInfo]."],["GetScreenResources","The `GetScreenResources` request."],["GetScreenResourcesCookie","Cookie type for [GetScreenResources]."],["GetScreenResourcesCookieUnchecked","Unchecked cookie type for [GetScreenResources]."],["GetScreenResourcesCurrent","The `GetScreenResourcesCurrent` request."],["GetScreenResourcesCurrentCookie","Cookie type for [GetScreenResourcesCurrent]."],["GetScreenResourcesCurrentCookieUnchecked","Unchecked cookie type for [GetScreenResourcesCurrent]."],["GetScreenResourcesCurrentReply","Reply type for [GetScreenResourcesCurrent]."],["GetScreenResourcesReply","Reply type for [GetScreenResources]."],["GetScreenSizeRange","The `GetScreenSizeRange` request."],["GetScreenSizeRangeCookie","Cookie type for [GetScreenSizeRange]."],["GetScreenSizeRangeCookieUnchecked","Unchecked cookie type for [GetScreenSizeRange]."],["GetScreenSizeRangeReply","Reply type for [GetScreenSizeRange]."],["Lease",""],["LeaseNotify",""],["ListOutputProperties","The `ListOutputProperties` request."],["ListOutputPropertiesCookie","Cookie type for [ListOutputProperties]."],["ListOutputPropertiesCookieUnchecked","Unchecked cookie type for [ListOutputProperties]."],["ListOutputPropertiesReply","Reply type for [ListOutputProperties]."],["ListProviderProperties","The `ListProviderProperties` request."],["ListProviderPropertiesCookie","Cookie type for [ListProviderProperties]."],["ListProviderPropertiesCookieUnchecked","Unchecked cookie type for [ListProviderProperties]."],["ListProviderPropertiesReply","Reply type for [ListProviderProperties]."],["Mode",""],["ModeFlag",""],["ModeInfo",""],["MonitorInfo",""],["MonitorInfoBuf",""],["MonitorInfoIterator",""],["NotifyEvent","The `NotifyEvent` event."],["NotifyMask",""],["Output",""],["OutputChange",""],["OutputProperty",""],["Provider",""],["ProviderCapability",""],["ProviderChange",""],["ProviderProperty",""],["QueryOutputProperty","The `QueryOutputProperty` request."],["QueryOutputPropertyCookie","Cookie type for [QueryOutputProperty]."],["QueryOutputPropertyCookieUnchecked","Unchecked cookie type for [QueryOutputProperty]."],["QueryOutputPropertyReply","Reply type for [QueryOutputProperty]."],["QueryProviderProperty","The `QueryProviderProperty` request."],["QueryProviderPropertyCookie","Cookie type for [QueryProviderProperty]."],["QueryProviderPropertyCookieUnchecked","Unchecked cookie type for [QueryProviderProperty]."],["QueryProviderPropertyReply","Reply type for [QueryProviderProperty]."],["QueryVersion","The `QueryVersion` request."],["QueryVersionCookie","Cookie type for [QueryVersion]."],["QueryVersionCookieUnchecked","Unchecked cookie type for [QueryVersion]."],["QueryVersionReply","Reply type for [QueryVersion]."],["RefreshRates",""],["RefreshRatesBuf",""],["RefreshRatesIterator",""],["ResourceChange",""],["Rotation",""],["ScreenChangeNotifyEvent","The `ScreenChangeNotifyEvent` event."],["ScreenSize",""],["SelectInput","The `SelectInput` request."],["SetCrtcConfig","The `SetCrtcConfig` request."],["SetCrtcConfigCookie","Cookie type for [SetCrtcConfig]."],["SetCrtcConfigCookieUnchecked","Unchecked cookie type for [SetCrtcConfig]."],["SetCrtcConfigReply","Reply type for [SetCrtcConfig]."],["SetCrtcGamma","The `SetCrtcGamma` request."],["SetCrtcTransform","The `SetCrtcTransform` request."],["SetMonitor","The `SetMonitor` request."],["SetOutputPrimary","The `SetOutputPrimary` request."],["SetPanning","The `SetPanning` request."],["SetPanningCookie","Cookie type for [SetPanning]."],["SetPanningCookieUnchecked","Unchecked cookie type for [SetPanning]."],["SetPanningReply","Reply type for [SetPanning]."],["SetProviderOffloadSink","The `SetProviderOffloadSink` request."],["SetProviderOutputSource","The `SetProviderOutputSource` request."],["SetScreenConfig","The `SetScreenConfig` request."],["SetScreenConfigCookie","Cookie type for [SetScreenConfig]."],["SetScreenConfigCookieUnchecked","Unchecked cookie type for [SetScreenConfig]."],["SetScreenConfigReply","Reply type for [SetScreenConfig]."],["SetScreenSize","The `SetScreenSize` request."],["Transform",""]]});