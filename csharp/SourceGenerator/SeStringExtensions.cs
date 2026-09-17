using System.Text;
using Lumina.Text.Payloads;
using Lumina.Text.ReadOnly;

namespace SourceGenerator;

internal static class SeStringExtensions {
    internal static string TextValue(this ReadOnlySeString str) {
        var sb = new StringBuilder();
        foreach (var payload in str.AsSpan()) {
            switch (payload.Type) {
                case ReadOnlySePayloadType.Text:
                    sb.Append(Encoding.UTF8.GetString(payload.Body));
                    break;

                case ReadOnlySePayloadType.Macro when payload.MacroCode == MacroCode.Hyphen:
                    sb.Append('-');
                    break;

                case ReadOnlySePayloadType.Macro when payload.MacroCode == MacroCode.Num:
                    if (payload.TryGetExpression(out var valueExpression)
                        && valueExpression.TryGetUInt(out var value)
                        && value < 10) {
                        sb.Append((char) (48 + value));
                    }
                    break;
            }
        }

        return sb.ToString();
    }
}
