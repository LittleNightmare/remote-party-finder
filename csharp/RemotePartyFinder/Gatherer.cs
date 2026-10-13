using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Threading;
using System.Threading.Tasks;
using Dalamud.Game.Gui.PartyFinder.Types;
using Dalamud.Plugin.Services;
using Newtonsoft.Json;

namespace RemotePartyFinder;

internal class Gatherer : IDisposable
{
    private Plugin Plugin { get; }

    private ConcurrentDictionary<int, List<IPartyFinderListing>> Batches { get; } = new();
    private Stopwatch UploadTimer { get; } = new();
    private HttpClient Client { get; } = new();
    private readonly CancellationTokenSource CancelSource = new();
    private readonly object UploadLock = new();
    private readonly List<Task> Uploads = new();
    private bool Disposed;

    internal Gatherer(Plugin plugin)
    {
        this.Plugin = plugin;

        this.UploadTimer.Start();

        this.Plugin.PartyFinderGui.ReceiveListing += this.OnListing;
        this.Plugin.Framework.Update += this.OnUpdate;
    }

    public void Dispose()
    {
        Task[] uploads;
        lock (this.UploadLock)
        {
            if (this.Disposed) return;
            this.Disposed = true;
            uploads = this.Uploads.ToArray();
            this.Batches.Clear();
        }
        this.Plugin.Framework.Update -= this.OnUpdate;
        this.Plugin.PartyFinderGui.ReceiveListing -= this.OnListing;
        this.CancelSource.Cancel();
        _ = this.CleanupAsync(uploads);
    }

    private async Task CleanupAsync(Task[] uploads)
    {
        try
        {
            await Task.WhenAll(uploads).ConfigureAwait(false);
        }
        catch
        {
            // Each upload observes and logs its own failure.
        }
        finally
        {
            this.Client.Dispose();
            this.CancelSource.Dispose();
        }
    }

    private void OnListing(IPartyFinderListing listing, IPartyFinderListingEventArgs args)
    {
        lock (this.UploadLock)
        {
            if (this.Disposed) return;
            this.Batches.GetOrAdd(args.BatchNumber, static _ => []).Add(listing);
        }
    }

    private void OnUpdate(IFramework framework1)
    {
        lock (this.UploadLock)
        {
            if (this.Disposed) return;
            if (this.UploadTimer.Elapsed < TimeSpan.FromSeconds(10))
            {
                return;
            }

            this.UploadTimer.Restart();

            foreach (var (batch, listings) in this.Batches.ToList())
            {
                this.Batches.Remove(batch, out _);
                this.Uploads.RemoveAll(task => task.IsCompleted);
                var urls = Plugin.Configuration.UploadUrls.Where(url => url.IsEnabled).Select(url => url.Url).ToArray();
                var snapshot = listings.ToArray();
                var token = this.CancelSource.Token;
                this.Uploads.Add(Task.Run(async () =>
                {
                    try
                    {
#if DEBUG
                        foreach (var listing in snapshot)
                        {
                            Plugin.Log.Debug($"Name: {listing.Name.TextValue}, Description: {listing.Description.TextValue}, Category: {listing.Category}, RawDuty: {listing.RawDuty}, DutyType: {listing.DutyType}");
                        }
#endif
                        var uploadable = snapshot
                            .Select(listing => new UploadableListing(listing))
                            .ToList();
                        var json = JsonConvert.SerializeObject(uploadable);

                        foreach (var uploadUrl in urls)
                        {
                            token.ThrowIfCancellationRequested();
                            try
                            {
                                using var content = new StringContent(json)
                                {
                                    Headers = { ContentType = MediaTypeHeaderValue.Parse("application/json") },
                                };
                                using var resp = await this.Client.PostAsync(uploadUrl, content, token).ConfigureAwait(false);
                                var output = await resp.Content.ReadAsStringAsync(token).ConfigureAwait(false);
                                Plugin.Log.Info($"{uploadUrl}:\n{output}");
                            }
                            catch (Exception error) when (error is not OperationCanceledException || !token.IsCancellationRequested)
                            {
                                LogUploadError(error, $"Upload to {uploadUrl} failed");
                            }
                        }
                    }
                    catch (OperationCanceledException) when (token.IsCancellationRequested)
                    {
                        try { Plugin.Log.Info("Upload task canceled"); } catch { }
                    }
                    catch (Exception error)
                    {
                        LogUploadError(error, "Upload task failed");
                    }
                }));
            }
        }
    }

    private static void LogUploadError(Exception error, string message)
    {
        try { Plugin.Log.Error(error, message); } catch { }
    }
}
