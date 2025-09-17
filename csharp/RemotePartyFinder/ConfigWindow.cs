using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;
using Dalamud.Interface.Components;
using Dalamud.Interface.Utility.Raii;
using Dalamud.Interface.Windowing;
using Dalamud.Bindings.ImGui;

namespace RemotePartyFinder;

public class ConfigWindow : Window, IDisposable
{
    private readonly Configuration _configuration;
    private string _uploadUrlTempString = string.Empty;
    private string _uploadUrlError = string.Empty;

    public ConfigWindow(Plugin plugin) : base("Remote Party Finder")
    {
        _configuration = plugin.Configuration;
        Flags = ImGuiWindowFlags.NoCollapse | ImGuiWindowFlags.AlwaysAutoResize;

        Size = new Vector2(500, 0);
    }

    public void Dispose()
    {
    }

    public override void OnClose()
    {
        _configuration.Save();
    }

    public override void Draw()
    {
        var isAdvanced = _configuration.AdvancedSettingsEnabled;
        ImGui.TextWrapped(
            "This section is for advanced users to configure which services to send party finder data to. " +
            "Only enable if you know what you are doing.");
        if (ImGui.Checkbox("Enable Advanced Settings", ref isAdvanced))
        {
            _configuration.AdvancedSettingsEnabled = isAdvanced;
            _configuration.Save();
        }

        if (!isAdvanced) return;

        
        using (ImRaii.Table((ImU8String)"uploadUrls", 3, ImGuiTableFlags.SizingFixedFit | ImGuiTableFlags.Borders))
        {
            ImGui.TableSetupColumn("#", ImGuiTableColumnFlags.WidthFixed);
            ImGui.TableSetupColumn("URL", ImGuiTableColumnFlags.WidthStretch);
            ImGui.TableSetupColumn("Enabled", ImGuiTableColumnFlags.WidthFixed);
            ImGui.TableHeadersRow();
            
            using var id = ImRaii.PushId((ImU8String)"urls");
            foreach (var (uploadUrl, index) in _configuration.UploadUrls.Select((url, index) => (url, index + 1)))
            {
                id.Push(index);

                ImGui.TableNextRow();
                ImGui.TableSetColumnIndex(0);
                ImGui.TextUnformatted(index.ToString());
                
                ImGui.TableSetColumnIndex(1);
                ImGui.TextUnformatted(uploadUrl.Url);

                ImGui.TableSetColumnIndex(2);
                var isEnabled = uploadUrl.IsEnabled;
                if (ImGui.Checkbox("##uploadUrlCheckbox", ref isEnabled))
                {
                    uploadUrl.IsEnabled = isEnabled;
                }

                if (!uploadUrl.IsDefault)
                {
                    ImGui.SameLine();
                    if (ImGuiComponents.IconButton(Dalamud.Interface.FontAwesomeIcon.Trash))
                    {
                        _configuration.UploadUrls = _configuration.UploadUrls.Remove(uploadUrl);
                    }
                }
                
                id.Pop();
            }
            
            ImGui.TableNextRow();
            ImGui.TableSetColumnIndex(1);
            ImGui.SetNextItemWidth(-1);
            ImGui.InputText("##uploadUrlInput", ref _uploadUrlTempString, 300);
            ImGui.TableNextColumn();

            if (!string.IsNullOrEmpty(_uploadUrlTempString) &&
                ImGuiComponents.IconButton(Dalamud.Interface.FontAwesomeIcon.Plus))
            {
                _uploadUrlTempString = _uploadUrlTempString.TrimEnd();

                if (_configuration.UploadUrls.Any(r =>
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
                    _configuration.UploadUrls = _configuration.UploadUrls.Add(new(_uploadUrlTempString));
                    _uploadUrlTempString = string.Empty;
                }
            }
        }

        ImGui.Dummy(new (0, 5));

        if (ImGui.Button("Reset To Default##uploadUrlDefault"))
        {
            ResetToDefault();
        }

        if (string.IsNullOrEmpty(_uploadUrlError)) return;
        
        ImGui.SameLine();
        ImGui.TextColored(new Vector4(1, 0, 0, 1), _uploadUrlError);
    }

    private void ResetToDefault()
    {
        _configuration.UploadUrls = Configuration.DefaultUploadUrls();
        _configuration.Save();
    }

    private static bool ValidUrl(string url)
        => Uri.TryCreate(url, UriKind.Absolute, out var uriResult)
           && (uriResult.Scheme == Uri.UriSchemeHttps || uriResult.Scheme == Uri.UriSchemeHttp);
}