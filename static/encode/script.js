let data = document.getElementById("data");
let submit = document.getElementById("submit-data");
let passwordBox = document.getElementById("password_box");
let output = document.getElementById("output");

function sendData(prams) {
    fetch("/encrypt", {
        method: "POST",
        body: prams
    }).then((responce) => {
        if (responce.ok) {
            return responce.text();
        } else {
            return "There has been an error";
        }
    }).then((data) => {
        output.textContent = data;
    })
}

submit.addEventListener("click", (e) => {
    e.preventDefault();

    sendData(new URLSearchParams({
        data: data.value,
        password: passwordBox.value
    }));
});


document.addEventListener('keydown', (event)=> {    
    if (event.key == "Enter") {
        sendData(new URLSearchParams({
            data: data.value,
            password: passwordBox.value
        }));
    }
});