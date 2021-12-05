import myInit, {shortenString, ShortenMode} from "char-count-golf"

myInit().then(() => {

    const inputArea = document.querySelector("#inputArea")
    const outputArea = document.querySelector("#outputArea")
    const inputCount = document.querySelector("#inputCount")
    const outputCount = document.querySelector("#outputCount")

    const update = () => {
        const s = inputArea.value
        const short = shortenString(s, ShortenMode.WithPunctuation)
        outputArea.innerText = short
        inputCount.innerText = s.length
        outputCount.innerText = short.length
    }
    inputArea.oninput = update

    update()
})