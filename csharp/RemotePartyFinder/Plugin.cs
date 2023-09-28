using Dalamud.IoC;
using Dalamud.Plugin;
using Dalamud.Plugin.Services;

namespace RemotePartyFinder;

public class Plugin : IDalamudPlugin {
    [PluginService]
    internal static IPluginLog Log { get; private set; }

    [PluginService]
    internal IFramework Framework { get; private init; }

    [PluginService]
    internal IPartyFinderGui PartyFinderGui { get; private init; }

    private Gatherer Gatherer { get; }

    public Plugin() {
        this.Gatherer = new Gatherer(this);
    }

    public void Dispose() {
        this.Gatherer.Dispose();
    }
}
