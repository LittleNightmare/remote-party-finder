using System;
using System.Collections.Generic;
using System.Linq;
using Dalamud.Game.Gui.PartyFinder.Types;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;

namespace RemotePartyFinder {
    [Serializable]
    [JsonObject(NamingStrategyType = typeof(SnakeCaseNamingStrategy))]
    internal class UploadableListing {
        public uint Id { get; }
        public uint ContentIdLower { get; }
        public byte[] Name { get; }
        public byte[] Description { get; }
        public ushort CreatedWorld { get; }
        public ushort HomeWorld { get; }
        public ushort CurrentWorld { get; }
        public DutyCategory Category { get; }
        public ushort Duty { get; }
        public DutyType DutyType { get; }
        public bool BeginnersWelcome { get; }
        public ushort SecondsRemaining { get; }
        public ushort MinItemLevel { get; }
        public byte NumParties { get; }
        public byte SlotsAvailable { get; }
        public uint LastServerRestart { get; }
        public ObjectiveFlags Objective { get; }
        public ConditionFlags Conditions { get; }
        public DutyFinderSettingsFlags DutyFinderSettings { get; }
        public LootRuleFlags LootRules { get; }
        public SearchAreaFlags SearchArea { get; }
        public List<UploadableSlot> Slots { get; }
        public List<byte> JobsPresent { get; }

        internal UploadableListing(PartyFinderListing listing) {
            this.Id = listing.Id;
            this.ContentIdLower = listing.ContentIdLower;
            this.Name = listing.Name.Encode();
            this.Description = listing.Description.Encode();
            this.CreatedWorld = (ushort) listing.World.Value.RowId;
            this.HomeWorld = (ushort) listing.HomeWorld.Value.RowId;
            this.CurrentWorld = (ushort) listing.CurrentWorld.Value.RowId;
            this.Category = listing.Category;
            this.Duty = listing.RawDuty;
            this.DutyType = listing.DutyType;
            this.BeginnersWelcome = listing.BeginnersWelcome;
            this.SecondsRemaining = listing.SecondsRemaining;
            this.MinItemLevel = listing.MinimumItemLevel;
            this.NumParties = listing.Parties;
            this.SlotsAvailable = listing.SlotsAvailable;
            this.LastServerRestart = listing.LastPatchHotfixTimestamp;
            this.Objective = listing.Objective;
            this.Conditions = listing.Conditions;
            this.DutyFinderSettings = listing.DutyFinderSettings;
            this.LootRules = listing.LootRules;
            this.SearchArea = listing.SearchArea;
            this.Slots = listing.Slots.Select(slot => new UploadableSlot(slot)).ToList();
            this.JobsPresent = listing.RawJobsPresent.ToList();
        }
    }

    [Serializable]
    [JsonObject(NamingStrategyType = typeof(SnakeCaseNamingStrategy))]
    internal class UploadableSlot {
        public JobFlags Accepting { get; }

        internal UploadableSlot(PartyFinderSlot slot) {
            this.Accepting = slot.Accepting.Aggregate((JobFlags) 0, (agg, flag) => agg | flag);
        }
    }
}
