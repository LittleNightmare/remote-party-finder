(function () {
    function setUpLanguage() {
        let language = document.getElementById('language');
        for (let elem of language.querySelectorAll('[data-value]')) {
            elem.addEventListener('click', () => {
                document.cookie = `lang=${encodeURIComponent(elem.dataset.value)};path=/;max-age=31536000;samesite=lax`;
                window.location.reload();
            });
        }
    }

    setUpLanguage();
})();
