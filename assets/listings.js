(function () {
    let select = document.getElementById('data-centre-filter');

    let data_centres = [];
    for (let elem of document.querySelectorAll('#listings > .listing')) {
        let centre = elem.dataset['centre'];
        if (data_centres.indexOf(centre) === -1) {
            data_centres.push(centre);
        }
    }

    data_centres.sort();
    for (let centre of data_centres) {
        let opt = document.createElement('option');
        opt.innerText = centre;
        select.appendChild(opt);
    }

    let options = {
        valueNames: [
            'duty',
            'creator',
            'description',
            {data: ['centre']},
        ],
    };
    let list = new List('container', options);

    select.addEventListener('change', () => {
        let centre = select.value;
        if (centre === 'All') {
            list.filter();
            return;
        }

        console.log(`looking for ${centre}`);

        list.filter(item => {
            console.log(item.values().centre === centre)
            return item.values().centre === centre;
            // console.log(item.elm.dataset['centre']);
            // return item.elm.dataset['centre'] === centre;
        });
    });
})();
