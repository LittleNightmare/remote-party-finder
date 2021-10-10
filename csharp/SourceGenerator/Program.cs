using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using Lumina;
using Lumina.Data;
using Lumina.Excel;
using Lumina.Excel.GeneratedSheets;
using Lumina.Text;

namespace SourceGenerator {
    internal class Program {
        private static void Main(string[] args) {
            var data = new Dictionary<Language, GameData>(4);
            foreach (var lang in Languages.Keys) {
                data[lang] = new GameData(args[0], new LuminaOptions {
                    DefaultExcelLanguage = lang,
                });
            }

            var prog = new Program(data);

            File.WriteAllText(Path.Join(args[1], "duties.rs"), prog.GenerateDuties());
            File.WriteAllText(Path.Join(args[1], "jobs.rs"), prog.GenerateJobs());
            File.WriteAllText(Path.Join(args[1], "roulettes.rs"), prog.GenerateRoulettes());
            File.WriteAllText(Path.Join(args[1], "worlds.rs"), prog.GenerateWorlds());
            File.WriteAllText(Path.Join(args[1], "territory_names.rs"), prog.GenerateTerritoryNames());
            File.WriteAllText(Path.Join(args[1], "auto_translate.rs"), prog.GenerateAutoTranslate());
            File.WriteAllText(Path.Join(args[1], "treasure_maps.rs"), prog.GenerateTreasureMaps());
        }

        private Dictionary<Language, GameData> Data { get; }

        private Program(Dictionary<Language, GameData> data) {
            this.Data = data;
        }

        private static StringBuilder DefaultHeader(bool localisedText = false) {
            var sb = new StringBuilder("use std::collections::HashMap;\n");

            if (localisedText) {
                sb.Append("use super::LocalisedText;\n");
            }

            return sb;
        }

        private static readonly Dictionary<Language, string> Languages = new() {
            [Language.English] = "en",
            [Language.Japanese] = "ja",
            [Language.German] = "de",
            [Language.French] = "fr",
        };

        private string? GetLocalisedStruct<T>(uint rowId, Func<T, SeString?> nameFunc, uint indent = 0, bool capitalise = false) where T : ExcelRow {
            var def = this.Data[Language.English].GetExcelSheet<T>()!.GetRow(rowId)!;
            var defName = nameFunc(def)?.TextValue();
            if (string.IsNullOrEmpty(defName)) {
                return null;
            }

            var sb = new StringBuilder();

            sb.Append("LocalisedText {\n");

            foreach (var (language, key) in Languages) {
                var row = this.Data[language].GetExcelSheet<T>(language)?.GetRow(rowId);
                var name = row == null
                    ? defName
                    : nameFunc(row)?.TextValue().Replace("\"", "\\\"");
                name ??= defName;
                if (capitalise) {
                    name = name[..1].ToUpperInvariant() + name[1..];
                }

                for (var i = 0; i < indent + 4; i++) {
                    sb.Append(' ');
                }

                sb.Append($"{key}: \"{name}\",\n");
            }

            for (var i = 0; i < indent; i++) {
                sb.Append(' ');
            }

            sb.Append('}');

            return sb.ToString();
        }

        private string GenerateDuties() {
            var sb = DefaultHeader(true);
            sb.Append('\n');

            sb.Append("#[derive(Debug)]\n");
            sb.Append("pub struct DutyInfo {\n");
            sb.Append("    pub name: LocalisedText,\n");
            sb.Append("    pub high_end: bool,\n");
            sb.Append("    pub content_kind: ContentKind,\n");
            sb.Append("}\n\n");

            sb.Append("#[derive(Debug, Clone, Copy)]\n");
            sb.Append("#[allow(unused)]\n");
            sb.Append("#[repr(u32)]\n");
            sb.Append("pub enum ContentKind {\n");
            foreach (var kind in this.Data[Language.English].GetExcelSheet<ContentType>()!) {
                var name = kind.Name.TextValue().Replace(" ", "");
                if (name.Length > 0) {
                    sb.Append($"    {name} = {kind.RowId},\n");
                }
            }

            sb.Append("    Other(u32),\n");
            sb.Append("}\n\n");

            sb.Append("impl ContentKind {\n");

            sb.Append("    fn from_u32(kind: u32) -> Self {\n");
            sb.Append("        match kind {\n");
            foreach (var kind in this.Data[Language.English].GetExcelSheet<ContentType>()!) {
                var name = kind.Name.TextValue().Replace(" ", "");
                if (name.Length > 0) {
                    sb.Append($"            {kind.RowId} => Self::{name},\n");
                }
            }

            sb.Append("            x => Self::Other(x),\n");
            sb.Append("        }\n");
            sb.Append("    }\n\n");

            sb.Append("    pub fn as_u32(self) -> u32 {\n");
            sb.Append("        match self {\n");
            foreach (var kind in this.Data[Language.English].GetExcelSheet<ContentType>()!) {
                var name = kind.Name.TextValue().Replace(" ", "");
                if (name.Length > 0) {
                    sb.Append($"            Self::{name} => {kind.RowId},\n");
                }
            }

            sb.Append("            Self::Other(x) => x,\n");
            sb.Append("        }\n");
            sb.Append("    }\n");

            sb.Append("}\n\n");

            sb.Append("lazy_static::lazy_static! {\n");
            sb.Append("    pub static ref DUTIES: HashMap<u32, DutyInfo> = maplit::hashmap! {\n");

            foreach (var cfc in this.Data[Language.English].GetExcelSheet<ContentFinderCondition>()!) {
                if (cfc.RowId == 0) {
                    continue;
                }

                var name = this.GetLocalisedStruct<ContentFinderCondition>(cfc.RowId, row => row.Name, 12, true);
                if (name == null) {
                    continue;
                }

                var highEnd = cfc.HighEndDuty ? "true" : "false";
                var contentType = cfc.ContentType.Value;
                var contentKind = contentType?.Name?.TextValue().Replace(" ", "");
                if (string.IsNullOrEmpty(contentKind)) {
                    contentKind = $"Other({contentType?.RowId ?? 0})";
                }

                sb.Append($"        {cfc.RowId} => DutyInfo {{\n");
                sb.Append($"            name: {name},\n");
                sb.Append($"            high_end: {highEnd},\n");
                sb.Append($"            content_kind: ContentKind::{contentKind},\n");
                sb.Append("        },\n");
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }

        private string GenerateJobs() {
            var sb = DefaultHeader();
            sb.Append("use ffxiv_types::jobs::{ClassJob, Class, Job, NonCombatJob};\n\n");
            sb.Append("lazy_static::lazy_static! {\n");
            sb.Append("    pub static ref JOBS: HashMap<u32, ClassJob> = maplit::hashmap! {\n");

            foreach (var cj in this.Data[Language.English].GetExcelSheet<ClassJob>()!) {
                if (cj.RowId == 0) {
                    continue;
                }

                var name = cj.NameEnglish.TextValue().Replace(" ", "");
                if (name.Length <= 0) {
                    continue;
                }

                var isCombat = cj.Role != 0;
                var isClass = cj.JobIndex == 0;

                string value;
                if (isCombat) {
                    value = isClass
                        ? $"ClassJob::Class(Class::{name})"
                        : $"ClassJob::Job(Job::{name})";
                } else {
                    value = $"ClassJob::NonCombat(NonCombatJob::{name})";
                }

                sb.Append($"        {cj.RowId} => {value},\n");
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }

        private string GenerateRoulettes() {
            var sb = DefaultHeader(true);
            sb.Append('\n');
            sb.Append("#[derive(Debug)]\n");
            sb.Append("pub struct RouletteInfo {\n");
            sb.Append("    pub name: LocalisedText,\n");
            sb.Append("    pub pvp: bool,\n");
            sb.Append("}\n\n");

            sb.Append("lazy_static::lazy_static! {\n");
            sb.Append("    pub static ref ROULETTES: HashMap<u32, RouletteInfo> = maplit::hashmap! {\n");

            foreach (var cr in this.Data[Language.English].GetExcelSheet<ContentRoulette>()!) {
                if (cr.RowId == 0) {
                    continue;
                }

                var name = this.GetLocalisedStruct<ContentRoulette>(cr.RowId, row => row.Name, 12);
                if (name == null) {
                    continue;
                }

                var pvp = cr.Unknown28 == 6
                    ? "true"
                    : "false";

                sb.Append($"        {cr.RowId} => RouletteInfo {{\n");
                sb.Append($"            name: {name},\n");
                sb.Append($"            pvp: {pvp},\n");
                sb.Append("        },\n");
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }

        private string GenerateWorlds() {
            var sb = DefaultHeader();
            sb.Append("use ffxiv_types::World;\n\n");
            sb.Append("lazy_static::lazy_static! {\n");
            sb.Append("    pub static ref WORLDS: HashMap<u32, World> = maplit::hashmap! {\n");

            foreach (var world in this.Data[Language.English].GetExcelSheet<World>()!) {
                if (world.RowId == 0 || !world.IsPublic || world.DataCenter.Row == 0) {
                    continue;
                }

                var name = world.Name.TextValue();
                if (name.Length <= 0) {
                    continue;
                }

                sb.Append($"        {world.RowId} => World::{name},\n");
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }

        private string GenerateTerritoryNames() {
            var sb = DefaultHeader(true);
            sb.Append("\nlazy_static::lazy_static! {\n");
            sb.Append("    pub static ref TERRITORY_NAMES: HashMap<u32, LocalisedText> = maplit::hashmap! {\n");

            foreach (var tt in this.Data[Language.English].GetExcelSheet<TerritoryType>()!) {
                if (tt.RowId == 0 || tt.PlaceName.Row == 0) {
                    continue;
                }

                var name = this.GetLocalisedStruct<TerritoryType>(
                    tt.RowId,
                    row => row.PlaceName.Value!.Name,
                    8
                );
                if (name == null) {
                    continue;
                }

                sb.Append($"        {tt.RowId} => {name},\n");
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }

        private string GenerateAutoTranslate() {
            var sb = DefaultHeader(true);
            sb.Append("\nlazy_static::lazy_static! {\n");
            sb.Append("    pub static ref AUTO_TRANSLATE: HashMap<(u32, u32), LocalisedText> = maplit::hashmap! {\n");

            foreach (var row in this.Data[Language.English].GetExcelSheet<Completion>()!) {
                var lookup = row.LookupTable.TextValue();
                if (lookup is not ("" or "@")) {
                    // TODO: do lookup
                } else {
                    var text = this.GetLocalisedStruct<Completion>(row.RowId, row => row.Text, 8);
                    if (text != null) {
                        sb.Append($"        ({row.Group}, {row.RowId}) => {text},\n");
                    }
                }
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }

        private string GenerateTreasureMaps() {
            var sb = DefaultHeader(true);
            sb.Append("\nlazy_static::lazy_static! {\n");
            sb.Append("    pub static ref TREASURE_MAPS: HashMap<u32, LocalisedText> = maplit::hashmap! {\n");
            sb.Append("        0 => LocalisedText {\n");
            sb.Append("            en: \"All Levels\",\n");
            sb.Append("            ja: \"レベルを指定しない\",\n");
            sb.Append("            de: \"Jede Stufe\",\n");
            sb.Append("            fr: \"Tous niveaux\",\n");
            sb.Append("        },\n");

            var i = 1;
            foreach (var row in this.Data[Language.English].GetExcelSheet<TreasureHuntRank>()!) {
                // IS THIS RIGHT?
                if (row.TreasureHuntTexture != 0) {
                    continue;
                }

                SeString? GetMapName(TreasureHuntRank thr) {
                    var name = thr.KeyItemName.Value?.Name;
                    return string.IsNullOrEmpty(name?.TextValue())
                        ? thr.ItemName.Value?.Name
                        : name;
                }

                var name = this.GetLocalisedStruct<TreasureHuntRank>(row.RowId, GetMapName, 8);
                if (!string.IsNullOrEmpty(name)) {
                    sb.Append($"        {i++} => {name},\n");
                }
            }

            sb.Append("    };\n");
            sb.Append("}\n");

            return sb.ToString();
        }
    }
}
