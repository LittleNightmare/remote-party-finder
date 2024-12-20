using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;
using Dalamud.Interface.Components;
using Dalamud.Interface.Utility.Raii;
using Dalamud.Interface.Windowing;
using ImGuiNET;

namespace RemotePartyFinder;

public class ConfigWindow : Window, IDisposable {
    private Configuration Configuration;
    private List<UploadUrl> uploadUrls = new();
    private bool uploadUrlsChanged;
    private string uploadUrlTempString = string.Empty;
    private string uploadUrlError = string.Empty;

    public ConfigWindow(Plugin plugin) : base("Remote Party Finder") {
        uploadUrlsChanged = false;
        Flags = ImGuiWindowFlags.NoCollapse | ImGuiWindowFlags.NoResize;
        Size = new Vector2(500, 250);
        Configuration = plugin.Configuration;

        uploadUrls = Configuration.UploadUrls.Select(x => x.Clone()).ToList();
    }

    public void Dispose() { }

    public override void OnClose() {
        uploadUrls = Configuration.UploadUrls.Select(x => x.Clone()).ToList();
    }

    public void Save() {
        Configuration.UploadUrls = uploadUrls.Select(x => x.Clone()).ToList();
        if (uploadUrlsChanged) {
            Configuration.Save();
            uploadUrlsChanged = false;
        }
        Toggle();
    }

    public override void Draw() {
        var isAdvanced = Configuration.AdvancedSettingsEnabled;
        ImGui.TextWrapped("This section is for advanced users to configure which services to send party finder data to. Only enable if you know what you are doing.");
        if (ImGui.Checkbox("Enable Advanced Settings", ref isAdvanced)) {
            Configuration.AdvancedSettingsEnabled = isAdvanced;
            Configuration.Save();
        }

        if (isAdvanced) {
            using var id = ImRaii.PushId("uploadUrls");
            ImGui.Columns(4);

            ImGui.SetColumnWidth(0, 28 * ImGui.GetIO().FontGlobalScale);
            ImGui.SetColumnWidth(1, ImGui.GetWindowContentRegionMax().X - ImGui.GetWindowContentRegionMin().X - (28 + 60 + 75) * ImGui.GetIO().FontGlobalScale);
            ImGui.SetColumnWidth(2, 60 * ImGui.GetIO().FontGlobalScale);
            ImGui.SetColumnWidth(3, 75 * ImGui.GetIO().FontGlobalScale);

            ImGui.Separator();

            ImGui.TextUnformatted("#");

            ImGui.NextColumn();
            ImGui.TextUnformatted("URL");

            ImGui.NextColumn();
            ImGui.TextUnformatted("Enabled");

            ImGui.NextColumn();
            ImGui.SetCursorPosX(ImGui.GetCursorPosX() + (ImGui.GetColumnWidth() - ImGui.CalcTextSize("Add/Delete").X) / 2 - 2);
            ImGui.TextUnformatted("Add/Delete");

            ImGui.NextColumn();
            ImGui.Separator();

            UploadUrl uploadUrlToRemove = null;

            var urlNumber = 1;

            foreach (var uploadUrl in uploadUrls) {
                var isEnabled = uploadUrl.IsEnabled;
                var isDefault = uploadUrl.IsDefault;

                id.Push(uploadUrl.Url);
                ImGui.TextUnformatted(urlNumber.ToString());

                ImGui.NextColumn();
                ImGui.TextUnformatted(uploadUrl.Url);

                ImGui.NextColumn();
                ImGui.SetCursorPosX(ImGui.GetCursorPosX() + (ImGui.GetColumnWidth() / 2) - 6 - (12 * ImGui.GetIO().FontGlobalScale));

                if (ImGui.Checkbox("##uploadUrlCheckbox", ref isEnabled)) {
                    this.uploadUrlsChanged = true;
                }

                ImGui.NextColumn();
                ImGui.SetCursorPosX(ImGui.GetCursorPosX() + (ImGui.GetColumnWidth() - (24 * ImGui.GetIO().FontGlobalScale)) / 2);

                if (!uploadUrl.IsDefault && ImGuiComponents.IconButton(Dalamud.Interface.FontAwesomeIcon.Trash)) {
                    uploadUrlToRemove = uploadUrl;
                }

                uploadUrl.IsEnabled = isEnabled;
                urlNumber++;
                id.Pop();

                ImGui.NextColumn();
                ImGui.Separator();
            }

            if (uploadUrlToRemove != null) {
                this.uploadUrls.Remove(uploadUrlToRemove);
                this.uploadUrlsChanged = true;
            }

            ImGui.TextUnformatted(urlNumber.ToString());

            ImGui.NextColumn();
            ImGui.SetNextItemWidth(-1);
            ImGui.InputText("##uploadUrlInput", ref uploadUrlTempString, 300);
            ImGui.NextColumn();

            ImGui.NextColumn();

            ImGui.SetCursorPosX(ImGui.GetCursorPosX() + (ImGui.GetColumnWidth() - (24 * ImGui.GetIO().FontGlobalScale)) / 2);
            if (!string.IsNullOrEmpty(uploadUrlTempString) && ImGuiComponents.IconButton(Dalamud.Interface.FontAwesomeIcon.Plus)) {
                uploadUrlTempString = uploadUrlTempString.TrimEnd();

                if (uploadUrls.Any(r => string.Equals(r.Url, uploadUrlTempString, StringComparison.InvariantCultureIgnoreCase))) {
                    uploadUrlError = "Endpoint already exists.";
                    Task.Delay(5000).ContinueWith(t => uploadUrlError = string.Empty);
                } else if (!ValidUrl(uploadUrlTempString)) {
                    this.uploadUrlError = "Invalid URL format.";
                    Task.Delay(5000).ContinueWith(t => uploadUrlError = string.Empty);
                } else {
                    uploadUrls.Add(new UploadUrl(uploadUrlTempString));
                    uploadUrlsChanged = true;
                    uploadUrlTempString = string.Empty;
                }
            }

            ImGui.NextColumn();
            ImGui.Separator();

            ImGui.Columns(1);
            if (!string.IsNullOrEmpty(uploadUrlError)) {
                ImGui.TextColored(new Vector4(1, 0, 0, 1), uploadUrlError);
            } else {
                if (ImGui.Button("Save Changes##uploadUrlSave")) {
                    Save();
                }
                ImGui.SameLine();
                if (ImGui.Button("Reset To Default##uploadUrlDefault")) {
                    ResetToDefault();
                }
            }
        }
    }

    private void ResetToDefault() {
        Configuration.UploadUrls.Clear();
        Configuration.Initialize();
        Configuration.Save();
        uploadUrlsChanged = false;
        uploadUrls = Configuration.UploadUrls.Select(x => x.Clone()).ToList();
    }

    private static bool ValidUrl(string url)
    => Uri.TryCreate(url, UriKind.Absolute, out var uriResult)
    && (uriResult.Scheme == Uri.UriSchemeHttps || uriResult.Scheme == Uri.UriSchemeHttp);
}
