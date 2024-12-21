namespace RemotePartyFinder;

public record UploadUrl(string Url)
{
    public string Url { get; set; } = Url;
    public bool IsDefault { get; init; }
    public bool IsEnabled { get; set; } = true;
}
