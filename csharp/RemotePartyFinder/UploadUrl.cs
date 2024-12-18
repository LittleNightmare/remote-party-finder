namespace RemotePartyFinder;

public class UploadUrl {
    public string Url { get; set; }
    public bool IsDefault { get; set; }
    public bool IsEnabled { get; set; }

    public UploadUrl(string url) {
        Url = url;
        IsDefault = false;
        IsEnabled = true;
    }

    public UploadUrl Clone() => this.MemberwiseClone() as UploadUrl;
}
