using System;
using System.Collections.Generic;
using Dalamud.Configuration;

namespace RemotePartyFinder;

[Serializable]
public class Configuration : IPluginConfiguration {
    public int Version { get; set; } = 1;
    public bool AdvancedSettingsEnabled = false;
    public List<UploadUrl> UploadUrls = [];

    public void Initialize()
    {
        if (UploadUrls.Count != 0) return;
        
        UploadUrls.Add(new UploadUrl("https://xivpf.com/contribute/multiple") { IsDefault = true });
        UploadUrls.Add(new UploadUrl("https://findingway.io/receiver") { IsDefault = true });
    }

    public void Save() {
        Plugin.PluginInterface.SavePluginConfig(this);
    }
}
