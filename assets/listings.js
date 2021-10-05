(function () {
    let stateWasNull = false;

    const state = {
        allowed: {},
        centre: "All",
        list: null,
    };

    const categoryDefaults = {
        type: 0,
        category: 0,
        highEnd: false,
        contentKind: -1,
    };

    function addJsClass() {
        document.children[0].className = 'js';
    }

    function saveLoadState() {
        let saved = localStorage.getItem('state');
        if (saved !== null) {
            try {
                saved = JSON.parse(saved);
            } catch (e) {
                saved = {}
                stateWasNull = true;
            }

            for (let key in saved) {
                state[key] = saved[key];
            }
        } else {
            stateWasNull = true;
        }

        window.addEventListener('pagehide', () => {
            let copy = {};
            for (let key in state) {
                if (key === 'list') {
                    continue;
                }

                copy[key] = state[key];
            }

            localStorage.setItem('state', JSON.stringify(copy));
        });
    }

    function getOptionValues(option) {
        return {
            type: option.dataset.type || categoryDefaults.type,
            category: option.dataset.category || categoryDefaults.category,
            highEnd: option.dataset.highEnd || categoryDefaults.highEnd,
            contentKind: option.dataset.contentKind || categoryDefaults.contentKind,
        };
    }

    function reflectState() {
        let category = document.getElementById('category-filter');
        for (let option of category.options) {
            let values = getOptionValues(option);
            let key = `${values.type}/${values.category}/${values.highEnd}`;
            if (state.allowed[key] === undefined) {
                if (stateWasNull) {
                    state.allowed[key] = [];
                } else {
                    continue;
                }
            }

            if (stateWasNull) {
                state.allowed[key].push(values.contentKind);
            }

            option.selected = state.allowed[key].includes(-1) || state.allowed[key].includes(values.contentKind);
        }

        let dataCentre = document.getElementById('data-centre-filter');
        dataCentre.value = state.centre;
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

    function refilter() {
        function categoryFilter(item) {
            let data = item.elm.dataset;
            let type = data.type;
            let category = data.category;
            let highEnd = data.highEnd;
            let contentKind = data.contentKind;

            let allowedContentKind = state.allowed[`${type}/${category}/${highEnd}`];
            if (allowedContentKind === undefined) {
                return false;
            }

            return allowedContentKind.includes(-1) || allowedContentKind.includes(contentKind);
        }

        function dataCentreFilter(item) {
            return state.centre === "All" || state.centre === item.values().centre;
        }

        state.list.filter(item => dataCentreFilter(item) && categoryFilter(item));
    }

    function setUpDataCentreFilter() {
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
            state.centre = select.value;
            refilter();
        });
    }

    function setUpCategoryFilter() {
        let select = document.getElementById('category-filter');

        select.addEventListener('change', () => {
            let allowed = new Map();

            for (let option of select.options) {
                if (!option.selected) {
                    continue;
                }

                let values = getOptionValues(option);

                let key = `${values.type}/${values.category}/${values.highEnd}`;
                if (allowed[key] === undefined) {
                    allowed[key] = [];
                }

                if (!allowed[key].includes(values.contentKind)) {
                    allowed[key].push(values.contentKind);
                }
            }

            state.allowed = allowed;
            refilter();
        });
    }

    addJsClass();
    saveLoadState();
    reflectState();
    state.list = setUpList();
    setUpDataCentreFilter();
    setUpCategoryFilter();
    refilter();
})();
