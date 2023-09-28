using System.Linq;
using Lumina.Text;
using Lumina.Text.Payloads;

namespace SourceGenerator; 

internal static class SeStringExtensions {
    internal static string TextValue(this SeString str) {
        var payloads = str.Payloads
            .Select(p => {
                if (p is TextPayload text) {
                    return p.Data[0] == 0x03 
                        ? text.RawString[1..]
                        : text.RawString;
                }

                if (p.Data.Length <= 1) {
                    return "";
                }

                if (p.Data[1] == 0x1F) {
                    return "-";
                }

                if (p.Data.Length > 2 && p.Data[1] == 0x20) {
                    var value = p.Data.Length > 4
                        ? p.Data[3] - 1
                        : p.Data[2];
                    return ((char) (48 + value)).ToString();
                }

                return "";
            });

        return string.Join("", payloads);
    }
}