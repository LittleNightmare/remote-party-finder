using Dalamud.Game;
using Dalamud.Game.Gui.PartyFinder;
using Dalamud.IoC;
using Dalamud.Plugin;

namespace RemotePartyFinder {
    public class Plugin : IDalamudPlugin {
        public string Name => "Remote Party Finder";

        [PluginService]
        internal Framework Framework { get; private init; }

        [PluginService]
        internal PartyFinderGui PartyFinderGui { get; private init; }

        private Gatherer Gatherer { get; }

        public Plugin() {
            this.Gatherer = new Gatherer(this);
        }

        public void Dispose() {
            this.Gatherer.Dispose();
        }
    }
}
