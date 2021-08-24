const xElem = document.getElementById("x");
const yElem = document.getElementById("y");

function mySubmit(evt) {
  if (!xElem.checkValidity() || !yElem.checkValidity()) {
    return;
  }

  let x = xElem.value;
  let y = yElem.value;

  const xhr = new XMLHttpRequest();
  xhr.onreadystatechange = function () {
    if (this.readyState !== XMLHttpRequest.DONE) return;
    const res = xhr.response;
    if (this.status !== 200) {
      return alert("Server error " + res);
    }
    alert(res);
  };
  xhr.open("post", "/api/sum", true);
  xhr.send(`x=${x}&y=${y}`);
}
