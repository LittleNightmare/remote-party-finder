using System.Collections.Generic;
using Pidgin;
using static Pidgin.Parser;
using static Pidgin.Parser<char>;

namespace SourceGenerator; 

internal static class AutoTranslate {
    internal static Parser<char, (string name, Maybe<IEnumerable<ISelectorPart>> selector)> Parser() {
        var sheetName = Any
            .AtLeastOnceUntil(Lookahead(Char('[').IgnoreResult().Or(End)))
            .Select(string.Concat)
            .Labelled("sheetName");

        var numPair = Map(
                (first, second) => (ISelectorPart) new IndexRange(
                    uint.Parse(string.Concat(first)),
                    uint.Parse(string.Concat(second))
                ),
                Digit.AtLeastOnce().Before(Char('-')),
                Digit.AtLeastOnce()
            )
            .Labelled("numPair");
        var singleRow = Digit
            .AtLeastOnce()
            .Select(string.Concat)
            .Select(num => (ISelectorPart) new SingleRow(uint.Parse(num)));
        var column = String("col-")
            .Then(Digit.AtLeastOnce())
            .Select(string.Concat)
            .Select(num => (ISelectorPart) new ColumnSpecifier(uint.Parse(num)));
        var noun = String("noun")
            .Select(_ => (ISelectorPart) new NounMarker());

        var selectorItems = OneOf(
                Try(numPair),
                singleRow,
                column,
                noun
            )
            .Separated(Char(','))
            .Labelled("selectorItems");
        var selector = selectorItems
            .Between(Char('['), Char(']'))
            .Labelled("selector");

        return Map(
            (name, selector) => (name, selector),
            sheetName,
            selector.Optional()
        );
    }
}

internal interface ISelectorPart {
}

internal class SingleRow : ISelectorPart {
    public uint Row { get; }

    public SingleRow(uint row) {
        this.Row = row;
    }
}

internal class IndexRange : ISelectorPart {
    public uint Start { get; }
    public uint End { get; }

    public IndexRange(uint start, uint end) {
        this.Start = start;
        this.End = end;
    }
}

internal class NounMarker : ISelectorPart {
}

internal class ColumnSpecifier : ISelectorPart {
    public uint Column { get; }

    public ColumnSpecifier(uint column) {
        this.Column = column;
    }
}
