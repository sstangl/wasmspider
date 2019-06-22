var testIndex = -1;
var currentRepeat = -1;
var repeatCount = 10;
var warmupMS = 8;

var output = [];
output.length = repeatCount;
for (var i = 0; i < output.length; i++) {
    output[i] = {};
}

function warmup()
{
    for (var start = new Date; new Date - start < warmupMS; ) {
        for (var i = 0; i < 100; ++i) {
            if (Math.atan(Math.acos(Math.asin(Math.random()))) > 4) // Always false.
                return;
        }
    }
}

function start() 
{
    window.setTimeout(next, 128);
}

function gen_test_html(test_number) {
    // The SunSpider names are kept in their original form,
    // but the functions have underscores instead of dashes.
    var function_name = tests[test_number].replace(/-/g, "_");

    var s = `
        function record(time) {
            document.getElementById("console").innerHTML = time + "ms";
            parent.recordResult(time);
        }

        window.onerror = function(e) {
            console.log("${function_name} failed with error: " + e);
            record(0 / 0);
        }

        function run(fn) {
            var _sunSpiderStartDate = new Date();
            fn();
            var interval = new Date() - _sunSpiderStartDate;
            record(interval);
        }

        fetch("wasmspider.wasm").then(response =>
            response.arrayBuffer()
        ).then(bytes =>
            WebAssembly.instantiate(bytes)
        ).then(result =>
            run(result.instance.exports.${function_name})
        );
    `;

    var html = `
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset=utf8>
                <title>WasmSpider ${tests[test_number]}</title>
                <link rel="stylesheet" href="sunspider.css">
            </head>
            <body>
                <h3>${tests[test_number]}</h3>
                <div id="console"></div>
                <script>
                    ${s}
                </script>
            </body>
        </html>
    `;

    return html;
}

function next()
{
    document.getElementById("frameparent").innerHTML = "";
    document.getElementById("frameparent").innerHTML = "<iframe id='testframe'>";
    var testFrame = document.getElementById("testframe");
    if (++testIndex < tests.length) {
        // Warm up the CPU a little bit, in case power management clocked it down
        // or put it to sleep. We're trying to strike a balance here: do enough
        // work so that all browsers see the CPU at an equal clock rate, but
        // not so much work that we hide performance problems caused by overly
        // aggressive power management.
        warmup();

        testFrame.contentDocument.open();
        testFrame.contentDocument.write(gen_test_html(testIndex));
        testFrame.contentDocument.close();

        // The Wasm loader is async.
        // Instead of setting the timeout here, do it after the result was recorded.
    } else if (++currentRepeat < repeatCount) { 
        document.getElementById("countdown").innerHTML = repeatCount - currentRepeat;
        testIndex = -1;

        window.setTimeout(next, 128);
    } else {
        finish();
    }
}

function recordResult(time)
{
    if (currentRepeat >= 0) // negative repeats are warmups
        output[currentRepeat][tests[testIndex]] = time;
    window.setTimeout(next, 0);
}

function finish()
{
    var outputString = "{";
    outputString += '"v": "wasmspider-0.0.1", ';
    for (var test in output[0]) {
        outputString += '"' + test + '":[';
        for (var i = 0; i < output.length; i++) {
            var time = output[i][test];
            if (time != time)
                time = "\"NaN\"";
            outputString += time + ",";
        }
        outputString = outputString.substring(0, outputString.length - 1);
        outputString += "],";
    }
    outputString = outputString.substring(0, outputString.length - 1);
    outputString += "}";

    location = "results.html?" + encodeURI(outputString);
}
