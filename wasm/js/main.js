import { entry, default_replace_rules } from "../pkg/index.js";


function runCommon(process) {
    // Clear output
    const output = document.getElementById("output");
    output.value = "";

    const source = document.getElementById("input").value;
    const rules = document.getElementById("rules").value;
    try{
        output.value = process(source, rules);
    }
    catch(e){
        output.value = e;
    }
}

document.getElementById("run").addEventListener("click", () => runCommon(entry));

document.getElementById("input").value = `$ \\frac{df}{dx} $`;

document.getElementById("rules").value = default_replace_rules();
