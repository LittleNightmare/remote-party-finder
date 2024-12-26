using System;
using System.Collections.Immutable;
using Dalamud.Configuration;

namespace RemotePartyFinder;

[Serializable]
public class Configuration : IPluginConfiguration {
    public int Version { get; set; } = 1;
    public bool AdvancedSettingsEnabled = false;
    public ImmutableList<UploadUrl> UploadUrls = DefaultUploadUrls();

    public static ImmutableList<UploadUrl> DefaultUploadUrls() => [
        new("https://xivpf.com/contribute/multiple") { IsDefault = true },
        new("https://findingway.io/receiver") { IsDefault = true }
    ];

    public void Save() {
        Plugin.PluginInterface.SavePluginConfig(this);
    }
}
