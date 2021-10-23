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

    function makeChart(tableId, chartId, chartType, combine = false) {
        let table = document.getElementById(tableId);
        let data = [];
        for (let row of table.querySelectorAll('tbody > tr')) {
            let cols = row.querySelectorAll('td');
            data.push({
                x: cols[0].innerText,
                y: Number(cols[1].innerText),
            });
        }

        if (combine) {
            data = combineTopN(data, 15);
        }

        new Chart(
            document.getElementById(chartId),
            {
                type: chartType,
                data: {
                    datasets: [{
                        data: data.map(entry => entry.y),
                        backgroundColor: colours,
                    }],
                    labels: data.map(entry => entry.x),
                },
                options: options,
            },
        );
    }

    makeChart('duties', 'dutiesChart', 'pie', true);
    makeChart('hosts', 'hostsChart', 'doughnut');
    makeChart('hours', 'hoursChart', 'bar');
    makeChart('days', 'daysChart', 'bar');
})();
