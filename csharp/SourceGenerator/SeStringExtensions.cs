using System.Linq;
using Lumina.Text;
using Lumina.Text.Payloads;

namespace SourceGenerator {
    internal static class SeStringExtensions {
        internal static string TextValue(this SeString str) {
            var payloads = str.Payloads
                .Select(p => {
                    if (p is TextPayload text) {
                        return text.RawString;
                    }

                    return p.Data.Length > 1 && p.Data[1] == 0x1F ? "-" : "";
                });
            // Produces:
            // MainCommand[col-,1-36,41-42,44-44,56-62,64-70,72-82,84-99]
            // TextCommand[col-,col-,1-31,33-211,213-321,323-455,459-611,613-629,632-632]
            // TextCommand[col-,col-,1-31,33-211,213-321,323-455,459-611,613-629,632-632]
            // Pet[,1-8,10-10,14-15,17-19]
            // PetMirage[col-♥]
            //
            // foreach (var payload in str.Payloads) {
            //     if (payload is TextPayload) {
            //         continue;
            //     }
            //
            //     if (payload.Data.Length > 1 && payload.Data[1] == 0x20) {
            //         Console.WriteLine(string.Join("", payloads));
            //     }
            // }

            return string.Join("", payloads);
        }
    }
}
