import { entry } from "../pkg/index.js";


function runCommon(process) {
    // Clear output
    const output = document.getElementById("output");
    output.value = "";

    const source = document.getElementById("input").value;
    try{
        output.value = process(source);
    }
    catch(e){
        output.value = e;
    }
}

document.getElementById("run").addEventListener("click", () => runCommon(entry));

document.getElementById("input").value = `$ \\frac{df}{dx} $`;
