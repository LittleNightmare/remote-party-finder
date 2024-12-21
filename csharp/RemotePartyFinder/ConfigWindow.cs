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

public class ConfigWindow : Window, IDisposable
{
    private readonly Configuration _configuration;
    private List<UploadUrl> _uploadUrls;
    private bool _uploadUrlsChanged;
    private string _uploadUrlTempString = string.Empty;
    private string _uploadUrlError = string.Empty;

    public ConfigWindow(Plugin plugin) : base("Remote Party Finder")
    {
        _uploadUrlsChanged = false;
        Flags = ImGuiWindowFlags.NoCollapse | ImGuiWindowFlags.NoResize;
        Size = new Vector2(500, 250);
        _configuration = plugin.Configuration;

        _uploadUrls = _configuration.UploadUrls.ToList();
    }

    public void Dispose()
    {
    }

    public override void OnClose()
    {
        _uploadUrls = _configuration.UploadUrls.ToList();
    }

    private void Save()
    {
        _configuration.UploadUrls = _uploadUrls.ToList();
        if (_uploadUrlsChanged)
        {
            _configuration.Save();
            _uploadUrlsChanged = false;
        }

        Toggle();
    }

    public override void Draw()
    {
        var isAdvanced = _configuration.AdvancedSettingsEnabled;
        ImGui.TextWrapped(
            "This section is for advanced users to configure which services to send party finder data to. Only enable if you know what you are doing.");
        if (ImGui.Checkbox("Enable Advanced Settings", ref isAdvanced))
        {
            _configuration.AdvancedSettingsEnabled = isAdvanced;
            _configuration.Save();
        }

        if (!isAdvanced) return;
        
        using var id = ImRaii.PushId("uploadUrls");
        ImGui.Columns(4);

        ImGui.SetColumnWidth(0, 28 * ImGui.GetIO().FontGlobalScale);
        ImGui.SetColumnWidth(1,
            ImGui.GetWindowContentRegionMax().X - ImGui.GetWindowContentRegionMin().X -
            (28 + 60 + 75) * ImGui.GetIO().FontGlobalScale);
        ImGui.SetColumnWidth(2, 60 * ImGui.GetIO().FontGlobalScale);
        ImGui.SetColumnWidth(3, 75 * ImGui.GetIO().FontGlobalScale);

        ImGui.Separator();

        ImGui.TextUnformatted("#");

        ImGui.NextColumn();
        ImGui.TextUnformatted("URL");

        ImGui.NextColumn();
        ImGui.TextUnformatted("Enabled");

        ImGui.NextColumn();
        ImGui.SetCursorPosX(ImGui.GetCursorPosX() +
            (ImGui.GetColumnWidth() - ImGui.CalcTextSize("Add/Delete").X) / 2 - 2);
        ImGui.TextUnformatted("Add/Delete");

        ImGui.NextColumn();
        ImGui.Separator();
        
        UploadUrl? uploadUrlToRemove = null;

        var urlNumber = 1;

        foreach (var uploadUrl in _uploadUrls)
        {
            var isEnabled = uploadUrl.IsEnabled;
            var isDefault = uploadUrl.IsDefault;

            id.Push(uploadUrl.Url);
            ImGui.TextUnformatted(urlNumber.ToString());

            ImGui.NextColumn();
            ImGui.TextUnformatted(uploadUrl.Url);

            ImGui.NextColumn();
            ImGui.SetCursorPosX(ImGui.GetCursorPosX() + (ImGui.GetColumnWidth() / 2) - 6 -
                                (12 * ImGui.GetIO().FontGlobalScale));

            if (ImGui.Checkbox("##uploadUrlCheckbox", ref isEnabled))
            {
                this._uploadUrlsChanged = true;
            }

            ImGui.NextColumn();
            ImGui.SetCursorPosX(ImGui.GetCursorPosX() +
                                (ImGui.GetColumnWidth() - (24 * ImGui.GetIO().FontGlobalScale)) / 2);

            if (!uploadUrl.IsDefault && ImGuiComponents.IconButton(Dalamud.Interface.FontAwesomeIcon.Trash))
            {
                uploadUrlToRemove = uploadUrl;
            }

            uploadUrl.IsEnabled = isEnabled;
            urlNumber++;
            id.Pop();

            ImGui.NextColumn();
            ImGui.Separator();
        }

        if (uploadUrlToRemove != null)
        {
            this._uploadUrls.Remove(uploadUrlToRemove);
            this._uploadUrlsChanged = true;
        }

        ImGui.TextUnformatted(urlNumber.ToString());

        ImGui.NextColumn();
        ImGui.SetNextItemWidth(-1);
        ImGui.InputText("##uploadUrlInput", ref _uploadUrlTempString, 300);
        ImGui.NextColumn();

        ImGui.NextColumn();

        ImGui.SetCursorPosX(ImGui.GetCursorPosX() +
                            (ImGui.GetColumnWidth() - (24 * ImGui.GetIO().FontGlobalScale)) / 2);
        if (!string.IsNullOrEmpty(_uploadUrlTempString) &&
            ImGuiComponents.IconButton(Dalamud.Interface.FontAwesomeIcon.Plus))
        {
            _uploadUrlTempString = _uploadUrlTempString.TrimEnd();

            if (_uploadUrls.Any(r =>
                    string.Equals(r.Url, _uploadUrlTempString, StringComparison.InvariantCultureIgnoreCase)))
            {
                _uploadUrlError = "Endpoint already exists.";
                Task.Delay(5000).ContinueWith(t => _uploadUrlError = string.Empty);
            }
            else if (!ValidUrl(_uploadUrlTempString))
            {
                this._uploadUrlError = "Invalid URL format.";
                Task.Delay(5000).ContinueWith(t => _uploadUrlError = string.Empty);
            }
            else
            {
                _uploadUrls.Add(new UploadUrl(_uploadUrlTempString));
                _uploadUrlsChanged = true;
                _uploadUrlTempString = string.Empty;
            }
        }

        ImGui.NextColumn();
        ImGui.Separator();

        ImGui.Columns(1);
        if (!string.IsNullOrEmpty(_uploadUrlError))
        {
            ImGui.TextColored(new Vector4(1, 0, 0, 1), _uploadUrlError);
        }
        else
        {
            if (ImGui.Button("Save Changes##uploadUrlSave"))
            {
                Save();
            }

            ImGui.SameLine();
            if (ImGui.Button("Reset To Default##uploadUrlDefault"))
            {
                ResetToDefault();
            }
        }
    }

    private void ResetToDefault()
    {
        _configuration.UploadUrls.Clear();
        _configuration.Initialize();
        _configuration.Save();
        _uploadUrlsChanged = false;
        _uploadUrls = _configuration.UploadUrls.ToList();
    }

    private static bool ValidUrl(string url)
        => Uri.TryCreate(url, UriKind.Absolute, out var uriResult)
           && (uriResult.Scheme == Uri.UriSchemeHttps || uriResult.Scheme == Uri.UriSchemeHttp);
}