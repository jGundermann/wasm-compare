import { Counter } from "wasmpack";

const counter = Counter.new();

const pre = document.getElementById("count")
const button = document.getElementById("add")

pre.innerText = counter.count();

button.onclick = event => {
    counter.add();
    pre.innerText = counter.count();
}