(function () {
    function addJsClass() {
        document.children[0].className = 'js';
    }

    function setUpList() {
        let options = {
            valueNames: [
                'duty',
                'creator',
                'description',
                {data: ['centre']},
            ],
        };
        return new List('container', options);
    }

    function setUpDataCentreFilter(list) {
        let select = document.getElementById('data-centre-filter');

        let data_centres = [];
        for (let elem of document.querySelectorAll('#listings > .listing')) {
            let centre = elem.dataset['centre'];
            if (!data_centres.includes(centre)) {
                data_centres.push(centre);
            }
        }

        data_centres.sort();
        for (let centre of data_centres) {
            let opt = document.createElement('option');
            opt.innerText = centre;
            select.appendChild(opt);
        }

        select.addEventListener('change', () => {
            let centre = select.value;
            if (centre === 'All') {
                list.filter();
                return;
            }

            list.filter(item => item.values().centre === centre);
        });
    }

    function setUpCategoryFilter(list) {
        let select = document.getElementById('category-filter');

        select.addEventListener('change', () => {
            let allowed = [];

            for (let option of select.options) {
                if (!option.selected) {
                    continue;
                }

                let type = option.dataset.type;
                let category = option.dataset.category;

                allowed.push(`${type}/${category}`);
            }

            list.filter(item => {
                let data = item.elm.dataset;
                let type = data.type;
                let category = data.category;

                return allowed.includes(`${type}/${category}`);
            });
        });
    }

    addJsClass();
    let list = setUpList();
    setUpDataCentreFilter(list);
    setUpCategoryFilter(list);
})();
