window.count_js = function (max, times) {
    for (let j = 0; j < times; ++j) {
        let res = 0;
        for (let i = 0; i < max; ++i) {
            res++;
        }
    }
}

window.substring_data = "foobarbazqux";

window.substring_js = function (times) {
    for (let i = 0; i < times; ++i) {
        window.substring_data.indexOf("baz");
    }
}

window.sort_array = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

window.sort_js = function (times) {
    for (let k = 0; k < times; ++k) {
        for (let i = 0; i < window.sort_array.length; ++i) {
            for (let j = i; j < window.sort_array.length; ++j) {
                 var left = window.sort_array[i];
                 var right = window.sort_array[j];
                 if (right < left) {
                     window.sort_array[i] = right;
                     window.sort_array[j] = left;
                 }
            }
        }
    }
}
