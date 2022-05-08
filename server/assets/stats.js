(function () {
    const colours = [
        '#D32F2F',
        '#1976D2',
        '#FBC02D',
        '#388E3C',
        '#7B1FA2',
        '#F57C00',
        '#5D4037',
        '#455A64',
        '#00796B',
        '#E64A19',
        '#C2185B',
        '#512DA8',
        '#0097A7',
    ];

    const options = {
        plugins: {
            tooltip: {
                callbacks: {
                    label: function (context) {
                        let total = context.dataset.data.reduce((total, next) => total + next);
                        let percentage = (context.raw / total * 100).toFixed(2);

                        return `${context.label} (${context.raw}, ${percentage}%)`;
                    }
                }
            },
            legend: {
                display: false,
            },
        },
    };

    function combineUnderMedian(data) {
        let midpoint = Math.trunc(data.length / 2);
        let wasOdd = data.length % 2 === 1;
        let median;
        if (wasOdd) {
            median = (data[midpoint].y + data[midpoint + 1].y) / 2;
        } else {
            median = data[midpoint].y;
        }

        let newData = [];
        let other = {
            x: 'Other',
            y: 0,
        };

        for (let entry of data) {
            if (entry.y <= median) {
                other.y += 1;
                continue;
            }

            newData.push(entry);
        }

        newData.push(other);

        return newData;
    }

    function combineTopN(data, n) {
        let newData = [];

        let other = {
            x: 'Other',
            y: 0,
        };

        for (let i = 0; i < data.length; i++) {
            if (i < n) {
                newData.push(data[i]);
                continue;
            }

            other.y += data[i].y;
        }

        newData.push(other);

        return newData;
    }

    function extractData(tableId, extractor = null) {
        if (extractor === null) {
            extractor = (cols) => {
                return {
                    label: cols[0].innerHTML,
                    value: Number(cols[1].innerHTML),
                };
            };
        }

        let table = document.getElementById(tableId);
        let data = [];
        for (let row of table.querySelectorAll('tbody > tr')) {
            let cols = row.querySelectorAll('td');
            data.push(extractor(cols));
        }

        return data;
    }

    function wrap(text) {
        text.each(function () {
            var text = d3.select(this),
                words = text.text().split(/\s+/).reverse(),
                word,
                line = [],
                lineNumber = 0,
                lineHeight = 1.1, // ems
                x = text.attr("x"),
                y = text.attr("y"),
                dy = 0, //parseFloat(text.attr("dy")),
                tspan = text.text(null)
                            .append("tspan")
                            .attr("x", x)
                            .attr("y", y)
                            .attr("dy", dy + "em");
            while (word = words.pop()) {
                line.push(word);
                tspan.text(line.join(" "));
                if (tspan.node().getComputedTextLength() > text.attr('width') - 6) {
                    line.pop();
                    tspan.text(line.join(" "));
                    line = [word];
                    tspan = text.append("tspan")
                                .attr("x", x)
                                .attr("y", y)
                                .attr("dy", ++lineNumber * lineHeight + dy + "em")
                                .text(word);
                }
            }
        });
    }

    function makeTreeMap(data, graphId, opts = {}) {
        let drawLabels = opts.drawLabels === undefined ? true : opts.drawLabels;
        let grouped = opts.grouped === undefined ? false : opts.grouped;

        let elem = document.getElementById(graphId);
        const [width, height] = [elem.offsetWidth, elem.offsetHeight];

        let svg = d3.select(`#${graphId}`)
                    .append('svg')
                    .attr('viewBox', `0 0 ${width} ${height}`);

        if (grouped) {
            d3.treemap()
              .size([width, height])
              .paddingInner(2)
              .paddingTop(4)
              .paddingRight(4)
              .paddingLeft(4)
              .paddingBottom(4)
            (data);
        } else {
            d3.treemap()
              .size([width, height])
              .paddingInner(2)
            (data);
        }

        let a = 0;
        let group = svg.selectAll('g')
                       .data(data.leaves())
                       .enter()
                       .append('g');

        let title = (d) => `${d.data.label} (${(d.data.value / d.parent.value * 100).toFixed(2)}% - ${d.data.value.toLocaleString()})`;

        group
            .append('title')
            .text(title);

        group
            .append('rect')
            .attr('x', d => d.x0)
            .attr('y', d => d.y0)
            .attr('width', d => d.x1 - d.x0)
            .attr('height', d => d.y1 - d.y0)
            .style('fill', d => {
                let colour = colours[a];
                a += 1;
                a %= colours.length;
                return colour;
            });

        if (drawLabels) {
            svg.selectAll('text')
               .data(data.leaves().filter(d => d.data.value / d.parent.value >= .05 ))
               .enter()
               .append('text')
               .attr('x', d => d.x0 + 5)
               .attr('y', d => d.y0 + 20)
               .attr('width', d => d.x1 - d.x0)
               .text(title)
               .attr('font-size', '1em')
               .attr('fill', 'white')
               .call(wrap);
        }

        if (grouped) {
            svg.selectAll('borders')
               .data(data.descendants().filter(d => d.depth === 1))
               .enter()
               .append('rect')
               .attr('x', d => d.x0)
               .attr('y', d => d.y0)
               .attr('width', d => d.x1 - d.x0)
               .attr('height', d => d.y1 - d.y0)
               .attr('fill', 'none')
               .attr('stroke', '#374956');

            svg.selectAll('titles')
               .data(data.descendants().filter(d => d.depth === 1))
               .enter()
               .append('text')
               .attr('x', d => d.x1 - 2)
               .attr('y', d => d.y1 - 2)
               .attr('text-anchor', 'end')
               .style('transform', d => {
                   if ((d.x1 - d.x0) < (d.y1 - d.y0)) {
                       return 'rotate(90deg)';
                   }

                   return null;
               })
               .style('transform-box', 'fill-box')
               .style('transform-origin', '95%')
               .text(d => d.data[0])
               .attr("font-size", d => {
                   if (d === data) {
                       return "1em";
                   }
                   let width = d.x1 - d.x0, height = d.y1 - d.y0;
                   return Math.max(Math.min(width/5, height/2, Math.sqrt((width*width + height*height))/10), 9)
               })
               .attr('fill', 'white');
        }
    }

    function makeBarPlot(data, graphId) {
        let elem = document.getElementById(graphId);
        const [marginLeft, marginRight, marginTop, marginBottom] = [100, 0, 16, 50];
        const [width, height] = [elem.offsetWidth - marginLeft - marginRight, elem.offsetHeight - marginTop - marginBottom];

        let svg = d3.select(`#${graphId}`)
                    .append('svg')
                    .attr('viewBox', `0 0 ${width + marginLeft + marginRight} ${height + marginTop + marginBottom}`)
                    .append('g')
                    .attr('transform', `translate(${marginLeft}, ${marginTop})`);

        let x = d3.scaleBand()
                  .range([0, width])
                  .domain(data.map(d => d.label))
                  .padding(0.2);
        svg.append('g')
           .attr('transform', `translate(0, ${height})`)
           .call(d3.axisBottom(x))
           .attr('font-size', '1em')
           .selectAll('text')
           .style('text-anchor', 'middle')
           .attr('font-size', '1em');

        let y = d3.scaleLinear()
                  .domain([0, data.map(d => d.value).reduce((max, a) => Math.max(max, a))])
                  .range([height, 0]);
        svg.append('g')
           .call(d3.axisLeft(y))
           .attr('font-size', '1em')
           .selectAll('text')
           .attr('font-size', '1em');

        let sum = data.map(d => d.value).reduce((total, a) => total + a);
        let colourIdx = 0;
        let group = svg.selectAll('mybar')
                       .data(data)
                       .enter()
                       .append('g');
        group.append('title')
             .text(d => `${d.value} (${(d.value / sum * 100).toFixed(2)}%)`);
        group.append('rect')
             .attr('x', d => x(d.label))
             .attr('y', d => y(d.value))
             .attr('width', x.bandwidth())
             .attr('height', d => height - y(d.value))
             .attr('fill', d => {
                 let colour = colours[colourIdx];
                 colourIdx += 1;
                 colourIdx %= colours.length;
                 return colour;
             });
    }

    makeTreeMap(
        d3.hierarchy({
            children: extractData('duties'),
        }).sum(d => d.value),
        'dutiesChart',
    );
    makeTreeMap(
        d3.hierarchy(
            d3.group(
                extractData(
                    'hosts',
                    (cols) => {
                        return {
                            label: cols[1].innerHTML,
                            world: cols[0].innerHTML,
                            value: Number(cols[2].innerHTML),
                        };
                    },
                ),
                d => d.world,
            )
        ).sum(d => d.value),
        'hostsChart',
        {
            drawLabels: false,
            grouped: true,
        },
    );
    makeBarPlot(
        extractData('hours'),
        'hoursChart',
    );
    makeBarPlot(
        extractData('days'),
        'daysChart',
    );
})();
